#version 440

layout(location = 0) in vec2 v_in_Pos;


void main() {
    int x = int(gl_InstanceIndex) % 24;
    int y = (int(gl_InstanceIndex) /24) % 24;
    gl_Position = vec4((v_in_Pos.x + (x)) / 100, (v_in_Pos.y + y) / 100, 0.0, 1.0);
}
