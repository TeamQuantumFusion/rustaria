#version 440

void main() {
    float x = float(1 - int(gl_VertexIndex)) * 0.5;
    float y = float(int(gl_VertexIndex & 1) * 2 - 1) * 0.5;
    gl_Position = vec4(x, y, 0.0, 1.0);
}
