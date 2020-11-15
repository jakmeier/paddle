mod texture_config;
pub use texture_config::TextureConfig;

use crate::{ErrorMessage, JsError, PaddleResult};
use web_sys::{HtmlImageElement, WebGlRenderingContext, WebGlTexture};

/// 2D texture
#[derive(Debug)]
pub(crate) struct Texture {
    webgl_texture: WebGlTexture,
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
        // gl.tex_parameteri(
        //     WebGlRenderingContext::TEXTURE_2D,
        //     WebGlRenderingContext::TEXTURE_WRAP_S,
        //     WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        // );
        // gl.tex_parameteri(
        //     WebGlRenderingContext::TEXTURE_2D,
        //     WebGlRenderingContext::TEXTURE_WRAP_T,
        //     WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        // );

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
        Ok(Self { webgl_texture })
    }
    pub fn webgl_texture(&self) -> &WebGlTexture {
        &self.webgl_texture
    }
}
