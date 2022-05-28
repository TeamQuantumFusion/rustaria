#version 330

layout(location = 0) in vec2 position;
layout(location = 1) in vec3 color;

out vec3 v_color;

uniform float screen_y_ratio;
uniform float zoom;
uniform vec2 player_pos;

void main() {
    vec2 offset = position - player_pos;
    gl_Position = vec4(vec2(offset.x, offset.y * screen_y_ratio) / zoom, 1.0, 1.0);
    v_color = color;
}