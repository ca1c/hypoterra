#version 150

in vec2 v_uv;
in vec4 v_color;

uniform sampler2D u_texture;
uniform float u_red;
uniform float u_green;
uniform float u_blue;

out vec4 o_color;

void main() {
    o_color = vec4(0.705,0.728,0.735,1.00);
}
