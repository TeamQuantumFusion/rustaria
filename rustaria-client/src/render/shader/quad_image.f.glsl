#version 330

out vec4 FragColor;

in vec2 tex;

// uniform sampler2D atlas;

void main() {
   // FragColor = texture(atlas, tex);
    FragColor = vec4(tex, 1.0, 1.0);
}
