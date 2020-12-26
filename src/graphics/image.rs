use crate::graphics::Texture;
use crate::graphics::TextureConfig;
use crate::Transform;
use crate::Vector;
use std::{cell::Cell, rc::Rc};
use web_sys::WebGlTexture;

use web_sys::{HtmlImageElement, WebGlRenderingContext};

use crate::{Domain, ErrorMessage, JsError, NutsCheck, PaddleResult, Rectangle};

#[derive(Clone, Debug, PartialEq, Eq)]
///An image that can be drawn to the screen
pub struct Image {
    pub(crate) texture: Texture,
    // Region within the texture, in normalized Texture coordinates (UV) ranging from (0|0) to (1|1)
    pub(crate) region: Rectangle,
}

// Message sent after HTML image element finished loading and it is ready to be bound to WebGL context.
// A ImageLoader must be registered that handles these messages.
struct BindTextureMessage {
    payload: Rc<Cell<BindTexturePayload>>,
}
enum BindTexturePayload {
    Request(HtmlImageElement),
    Response(Texture),
    Done,
}
/// Register this for it to handle BindTextureMessage.
pub struct ImageLoader {
    gl: WebGlRenderingContext,
    texture_config: TextureConfig,
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
        let texture = match cell.take() {
            BindTexturePayload::Response(data) => data,
            _ => return Err(ErrorMessage::technical("Texture loading failed".to_owned())),
        };

        let region = Rectangle::new_sized((1.0, 1.0));
        Ok(Image { texture, region })
    }

    pub fn natural_width(&self) -> f32 {
        self.texture.texel_width * self.region.width()
    }

    pub fn natural_height(&self) -> f32 {
        self.texture.texel_height * self.region.height()
    }

    pub fn natural_size(&self) -> Vector {
        (self.natural_width(), self.natural_height()).into()
    }

    pub(crate) fn texture(&self) -> &WebGlTexture {
        &self.texture.webgl_texture()
    }

    /// Create a view into an existing image, using texel coordinates (number of pixels in source texture)
    pub fn subimage_texels(&self, rect: Rectangle) -> Image {
        let img = Image {
            texture: self.texture.clone(),
            region: Rectangle::new(
                (
                    self.region.pos.x + rect.pos.x / self.natural_width(),
                    self.region.pos.y + rect.pos.y / self.natural_height(),
                ),
                (rect.width(), rect.height()),
            ),
        };
        debug_assert!(img.region.x() <= 1.0);
        debug_assert!(img.region.y() <= 1.0);
        debug_assert!(img.region.x() >= 0.0);
        debug_assert!(img.region.y() >= 0.0);
        debug_assert!(img.region.width() <= 1.0);
        debug_assert!(img.region.height() <= 1.0);
        debug_assert!(img.region.width() >= 0.0);
        debug_assert!(img.region.height() >= 0.0);
        img
    }
    /// Create a view into an existing image, using texture coordinates (from 0.0 to 1.0 in both dimensions)
    pub fn subimage(&self, rect: Rectangle) -> Image {
        let img = Image {
            texture: self.texture.clone(),
            region: Rectangle::new(
                (
                    self.region.pos.x + rect.pos.x,
                    self.region.pos.y + rect.pos.y,
                ),
                (rect.width(), rect.height()),
            ),
        };
        debug_assert!(img.region.x() <= 1.0);
        debug_assert!(img.region.y() <= 1.0);
        debug_assert!(img.region.x() >= 0.0);
        debug_assert!(img.region.y() >= 0.0);
        debug_assert!(img.region.width() <= 1.0);
        debug_assert!(img.region.height() <= 1.0);
        debug_assert!(img.region.width() >= 0.0);
        debug_assert!(img.region.height() >= 0.0);
        img
    }

    /// Transformation to project the full texture coordinates space onto the selected region by this image.
    pub(crate) fn texture_transform(&self) -> Transform {
        // note: If Image and SubImage become different struct, only SubImage would actually need this function
        Transform::translate(self.region.pos) * Transform::scale(self.region.size())
    }
}

impl ImageLoader {
    pub fn register(gl: WebGlRenderingContext, texture_config: TextureConfig) {
        let activity = nuts::new_domained_activity(Self { gl, texture_config }, &Domain::Frame);
        activity.subscribe(move |a, msg: &BindTextureMessage| {
            if let BindTexturePayload::Request(el) = msg.payload.take() {
                if let Some(data) = Texture::new(&a.gl, &el, &a.texture_config).nuts_check() {
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
