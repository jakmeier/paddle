use std::{cell::Cell, rc::Rc};

use web_sys::{HtmlImageElement, WebGlRenderingContext, WebGlTexture};

use crate::{
    quicksilver_compat::Rectangle, Domain, ErrorMessage, JsError, NutsCheck, PaddleResult,
};

#[derive(Clone, Debug)]
///An image that can be drawn to the screen
pub struct Image {
    pub(crate) source: Rc<ImageData>,
    pub(crate) region: Rectangle,
}

#[derive(Debug)]
pub struct ImageData {
    pub tex: WebGlTexture,
    pub el: HtmlImageElement,
    pub width: u32,
    pub height: u32,
}

// Message sent after HTML image element finished loading and it is ready to be bound to WebGL context.
// A ImageLoader must be registered that handles these messages.
struct BindTextureMessage {
    payload: Rc<Cell<BindTexturePayload>>,
}
enum BindTexturePayload {
    Request(HtmlImageElement),
    Response(ImageData),
    Done,
}
/// Register this for it to handle BindTextureMessage.
pub struct ImageLoader {
    gl: WebGlRenderingContext,
}

impl ImageData {
    fn new(gl: &WebGlRenderingContext, el: HtmlImageElement) -> PaddleResult<Self> {
        let width = el.width();
        let height = el.height();
        let tex = gl.create_texture().ok_or(ErrorMessage::technical(
            "Failed to create texture".to_owned(),
        ))?;
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&tex));
        // JS equivalent: texImage2D()
        gl.tex_image_2d_with_u32_and_u32_and_image(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGBA as i32,
            WebGlRenderingContext::RGBA,
            WebGlRenderingContext::UNSIGNED_BYTE,
            &el,
        )
        .map_err(JsError::from_js_value)?;
        Ok(Self {
            tex,
            el,
            width,
            height,
        })
    }
}

impl Image {
    pub async fn load(src: &str) -> PaddleResult<Self> {
        // Let the browser handle the image loading
        let el = HtmlImageElement::new().map_err(JsError::from_js_value)?;
        el.set_src(src);
        // asynchronously load data and block the future
        let promise = el.decode();
        wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(JsError::from_js_value)?;

        // When the image is ready, create a WebGL texture from it in the image loader and place it in a cell.
        let cell = Rc::new(Cell::new(BindTexturePayload::Request(el)));
        let msg = BindTextureMessage {
            payload: cell.clone(),
        };
        nuts::publish_awaiting_response(msg).await;

        // Then get that bounded texture which has been created
        let data = match cell.take() {
            BindTexturePayload::Response(data) => data,
            _ => return Err(ErrorMessage::technical("Image loading failed".to_owned())),
        };

        let region = Rectangle::new_sized((128, 128));
        Ok(Image {
            source: Rc::new(data),
            region,
        })
    }
}

impl ImageLoader {
    pub fn register(gl: WebGlRenderingContext) {
        let activity = nuts::new_domained_activity(Self { gl }, &Domain::Frame);
        activity.subscribe(move |a, msg: &BindTextureMessage| {
            if let BindTexturePayload::Request(el) = msg.payload.take() {
                if let Some(data) = ImageData::new(&a.gl, el).nuts_check() {
                    msg.payload.replace(BindTexturePayload::Response(data));
                }
            }
        })
    }
}

impl Default for BindTexturePayload {
    fn default() -> Self {
        Self::Done
    }
}
