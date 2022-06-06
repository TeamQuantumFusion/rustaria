#version 330

in vec2 pos;
in vec2 tex;

out vec2 v_tex_coords;

uniform float screen_y_ratio;
uniform float scale;
uniform vec2 player_pos;

void main() {
    vec2 offset = pos - player_pos;
    gl_Position = vec4(vec2(offset.x, offset.y * screen_y_ratio) / scale, 1.0, 1.0);
    v_tex_coords = tex;
}