mod texture_config;
mod image;
pub use texture_config::TextureConfig;
pub use image::*;

use crate::{ErrorMessage, JsError, PaddleResult};
use web_sys::{HtmlImageElement, WebGlRenderingContext, WebGlTexture};

/// 2D texture
#[derive(Debug, Clone)]
pub(crate) struct Texture {
    webgl_texture: WebGlTexture,
    pub(crate) texel_width: f32,
    pub(crate) texel_height: f32,
}

impl Texture {
    /// Upload a new texture to the GPU
    pub fn new(
        gl: &WebGlRenderingContext,
        img: &HtmlImageElement,
        config: &TextureConfig,
    ) -> PaddleResult<Self> {
        let webgl_texture = gl
            .create_texture()
            .ok_or_else(|| ErrorMessage::technical("Failed to create texture".to_owned()))?;
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&webgl_texture));

        // Clamp to edge allows using non-power-of-two sized texture sources. Always use that to keep it simple. (for now)
        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_S,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_T,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );

        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
            config.minification_filter.webgl_num(&config.mipmap_level),
        );
        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MAG_FILTER,
            config.magnification_filter.webgl_num(),
        );
        let color_format = config.color_format.webgl_num();

        // JS equivalent: texImage2D()
        gl.tex_image_2d_with_u32_and_u32_and_image(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            color_format,
            color_format as u32,
            WebGlRenderingContext::UNSIGNED_BYTE,
            &img,
        )
        .map_err(JsError::from_js_value)?;

        if config.mipmap_level.on() {
            gl.generate_mipmap(WebGlRenderingContext::TEXTURE_2D);
        }

        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);

        let texel_width = img.width() as f32;
        let texel_height = img.height() as f32;
        Ok(Self {
            webgl_texture,
            texel_width,
            texel_height,
        })
    }
    pub fn webgl_texture(&self) -> &WebGlTexture {
        &self.webgl_texture
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.webgl_texture == other.webgl_texture
    }
}
impl Eq for Texture {}

// This could be done if a single reference to the Texture was kept. Currently this is clones all over the place...
// use wasm_bindgen::JsCast;
// impl Drop for Texture {
//     fn drop(&mut self) {
//         if let Some(document) = web_sys::window().and_then(|w| w.document()) {
//             if let Some(canvas) = document.get_elements_by_tag_name("canvas").item(0) {
//                 let canvas: web_sys::HtmlCanvasElement =
//                     canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
//                 if let Some(gl) = canvas
//                     .get_context("webgl")
//                     .ok()
//                     .flatten()
//                     .and_then(|ctx| ctx.dyn_into::<WebGlRenderingContext>().ok())
//                 {
//                     gl.delete_texture(Some(&self.webgl_texture));
//                 }
//             }
//         }
//     }
// }
