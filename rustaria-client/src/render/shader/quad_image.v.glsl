#version 330 core


layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aTex;

out vec2 tex;

uniform float screen_y_ratio;

void main() {
    gl_Position = vec4(aPos.x, aPos.y * screen_y_ratio, 1.0, 1.0);
    tex = aTex;
}
