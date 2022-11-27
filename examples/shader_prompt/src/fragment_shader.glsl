varying highp vec2 Tex_coord; // This is stretched across the full river area, but the actual texture is tiled.
varying highp vec2 Coordinate; // This is the position in the full screen
uniform sampler2D sampler;
uniform mediump float Time;
void main() {
    highp float speed = 0.5;
    highp float waves = 0.25;
    highp float tiles = 20.0;
    highp vec2 tex_coord = mod(Tex_coord * tiles, 1.0);
    highp vec2 offset = vec2(waves * cos(Time+10.0*Coordinate.x)) + vec2(speed * Time, 0.0);
    offset.x += 2.0 * Tex_coord.y;
    offset.y += sin(length(Coordinate));
    highp vec2 p = mod(tex_coord + offset, 1.0);
    highp vec4 tex_color = texture2D(sampler, vec2(p.y, p.x));
    gl_FragColor = tex_color;
}