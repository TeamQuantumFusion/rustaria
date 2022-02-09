#version 330 core

out vec4 FragColor;

in vec2 f_tex;

uniform sampler2D atlas;

void main() {
    FragColor = texture(atlas, f_tex);
//    FragColor = vec4(tex, 1.0, 1.0);
}
