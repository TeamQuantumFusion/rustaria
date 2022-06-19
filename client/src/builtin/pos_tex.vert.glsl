#version 330

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 texture;

out vec2 v_texture;

uniform float screen_ratio;
uniform vec2 player_pos;
uniform float zoom;

void main() {
    vec2 pos = position - player_pos;
    gl_Position = vec4((pos.x * screen_ratio) / zoom, (pos.y) / zoom, 1.0, 1.0);
    v_texture = texture;
}
