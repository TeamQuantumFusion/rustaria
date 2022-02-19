#version 330 core

layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aTexture;

out vec2 f_tex;

uniform float screen_y_ratio;
uniform float zoom;
uniform vec2 pos;

void main() {
    vec2 vert = vec2(aPos.x, (aPos.y * screen_y_ratio));
    gl_Position = vec4((vert - pos) / zoom, 1.0, 1.0);
    f_tex = aTexture;
}
