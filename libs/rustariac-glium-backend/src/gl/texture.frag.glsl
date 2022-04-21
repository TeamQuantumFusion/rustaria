#version 330

in vec2 v_tex_coords;

out vec4 f_color;

uniform sampler2D atlas;

void main() {
    f_color = texture(atlas, v_tex_coords);
}