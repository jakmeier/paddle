pub const DEFAULT_VERTEX_SHADER: &str = r#"attribute vec3 position;
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

pub const DEFAULT_FRAGMENT_SHADER: &str = r#"varying highp vec4 Color;
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
