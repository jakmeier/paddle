use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::{ErrorMessage, PaddleResult};

pub fn new_vertex_shader(gl: &WebGlRenderingContext) -> PaddleResult<WebGlShader> {
    compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        DEFAULT_VERTEX_SHADER,
    )
}

pub fn new_fragment_shader(gl: &WebGlRenderingContext) -> PaddleResult<WebGlShader> {
    compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        DEFAULT_FRAGMENT_SHADER,
    )
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> PaddleResult<WebGlProgram> {
    let program = context
        .create_program()
        .ok_or_else(|| ErrorMessage::technical("Unable to create shader object".to_owned()))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);
    context.use_program(Some(&program));

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(ErrorMessage::technical(
            context
                .get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error creating program object".to_owned()),
        ))
    }
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

const DEFAULT_VERTEX_SHADER: &str = r#"attribute vec3 position;
attribute vec2 tex_coord;
attribute vec4 color;
attribute lowp float uses_texture;
varying vec2 Tex_coord;
varying vec4 Color;
varying lowp float Uses_texture;
uniform mat3 Projection;
void main() {
    vec3 projected = vec3(position.xy, 1.0) * Projection;
    gl_Position = vec4(projected.x / projected.z, projected.y / projected.z, position.z, 1.0);
    Tex_coord = tex_coord;
    Color = color;
    Uses_texture = uses_texture;
}"#;

const DEFAULT_FRAGMENT_SHADER: &str = r#"varying highp vec4 Color;
varying highp vec2 Tex_coord;
varying lowp float Uses_texture;
uniform sampler2D sampler;
void main() {
    highp vec4 tex_color = (int(Uses_texture) != 0) ? texture2D(sampler, Tex_coord) : vec4(1, 1, 1, 1);
    if (tex_color.a < 0.1) 
        discard;
    else
        gl_FragColor = Color * tex_color;
}"#;
