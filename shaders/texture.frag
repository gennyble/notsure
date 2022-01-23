#version 300 es
out mediump vec4 FragColor;

in mediump vec2 TexCoord;

uniform sampler2D Texture;

void main() {
    FragColor = texture(Texture, TexCoord);
}