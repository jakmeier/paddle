precision mediump float;
vec2 hash22(vec2 p)
{
	p = vec2(dot(p, vec2(127.1, 311.7)),
		dot(p, vec2(269.5, 183.3)));
	return -1.0 + 2.0 * fract(sin(p)*43758.5453123);
}
// Credits for perlin noise go to https://www.shadertoy.com/view/tdXBW4#
float perlin_noise(vec2 p)
{
	vec2 pi = floor(p);
	vec2 pf = p - pi;
	vec2 w = pf * pf * (3.0 - 2.0 * pf);
	return mix(mix(dot(hash22(pi + vec2(0.0, 0.0)), pf - vec2(0.0, 0.0)),
		dot(hash22(pi + vec2(1.0, 0.0)), pf - vec2(1.0, 0.0)), w.x),
		mix(dot(hash22(pi + vec2(0.0, 1.0)), pf - vec2(0.0, 1.0)),
			dot(hash22(pi + vec2(1.0, 1.0)), pf - vec2(1.0, 1.0)), w.x),
		w.y);
}
varying highp vec2 Coordinate;
uniform float Time;
void main() {
    vec2 pattern_noise_input = (sin(Time) *10.0 +30.0) * Coordinate;
    vec2 color_noise_input = Time + Coordinate;
    float c = perlin_noise(color_noise_input);
    vec3 color = vec3(c,0.5,1.0);
    float p = 0.2;
    gl_FragColor =  p * vec4(0.2,0.2,0.8,1.0) + (1.0-p) * vec4(color * perlin_noise(pattern_noise_input),1.0);
}