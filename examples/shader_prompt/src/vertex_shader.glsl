attribute vec3 position;
attribute vec2 tex_coord;
varying vec2 Coordinate;
uniform mat3 Projection;
varying vec2 Tex_coord;
void main() {
    // make the output area a flat river
    highp float flatten = 10.0;

    vec3 projected = vec3(position.xy, 1.0) * Projection;
    gl_Position = vec4(
        projected.x / projected.z,
        projected.y / projected.z / flatten,
        position.z, 1.0
    );
    Coordinate = projected.xy;
    Tex_coord = tex_coord;
}