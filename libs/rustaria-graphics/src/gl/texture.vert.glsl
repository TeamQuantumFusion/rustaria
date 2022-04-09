#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 texture;

out vec2 v_texture;

uniform float screen_y_ratio;
uniform float zoom;
uniform vec2 player_pos;

void main() {
    gl_Position = vec4((vec2(position.x, (position.y * screen_y_ratio)) - player_pos) / zoom, 1.0, 1.0);
    v_texture = texture;
}