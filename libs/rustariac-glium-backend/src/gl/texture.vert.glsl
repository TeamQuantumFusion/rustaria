#version 330

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

uniform float screen_y_ratio;
uniform float zoom;
uniform vec2 player_pos;

void main() {
    vec2 offset = position - player_pos;
    gl_Position = vec4(vec2(offset.x, offset.y * screen_y_ratio) / zoom, 1.0, 1.0);
    v_tex_coords = tex_coords;
}