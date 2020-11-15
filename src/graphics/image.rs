use crate::graphics::Texture;
use crate::graphics::TextureConfig;
use std::{cell::Cell, rc::Rc};

use web_sys::{HtmlImageElement, WebGlRenderingContext};

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
pub(crate) struct ImageData {
    pub tex: Texture,
    pub el: HtmlImageElement,
    pub width: i32,
    pub height: i32,
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
    texture_config: TextureConfig,
}

impl ImageData {
    fn new(
        gl: &WebGlRenderingContext,
        texture_config: &TextureConfig,
        el: HtmlImageElement,
    ) -> PaddleResult<Self> {
        let width = el.width() as i32;
        let height = el.height() as i32;
        let tex = Texture::new(gl, &el, texture_config)?;
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
    pub fn register(gl: WebGlRenderingContext, texture_config: TextureConfig) {
        let activity = nuts::new_domained_activity(Self { gl, texture_config }, &Domain::Frame);
        activity.subscribe(move |a, msg: &BindTextureMessage| {
            if let BindTexturePayload::Request(el) = msg.payload.take() {
                if let Some(data) = ImageData::new(&a.gl, &a.texture_config, el).nuts_check() {
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
