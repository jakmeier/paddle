//! Paddle supports only [WebGL 1.0](https://www.khronos.org/registry/webgl/specs/latest/1.0/) and hence shaders are written in [GLSL ES 1.0](https://www.khronos.org/registry/OpenGL/specs/es/2.0/GLSL_ES_Specification_1.00.pdf).
//! This makes it run is as many browsers as possible. ([See caniuse.com](https://caniuse.com/?search=webgl))
//! The downside is that many useful features are not available or not guaranteed to be available.
//!

mod custom_shader;
mod default_shaders;
mod uniform;
pub use custom_shader::*;
pub use default_shaders::*;
pub use uniform::*;

use crate::{ErrorMessage, PaddleResult};
use web_sys::{WebGlRenderingContext, WebGlShader};

pub fn new_vertex_shader(gl: &WebGlRenderingContext, text: &str) -> PaddleResult<WebGlShader> {
    compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, text)
}

pub fn new_fragment_shader(gl: &WebGlRenderingContext, text: &str) -> PaddleResult<WebGlShader> {
    compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, text)
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> PaddleResult<WebGlShader> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| ErrorMessage::technical("Unable to create shader object".to_owned()))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(ErrorMessage::technical(
            gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".to_owned()),
        ))
    }
}
