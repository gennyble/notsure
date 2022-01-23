#version 300 es
layout (location = 0) in vec3 ModelPosition;
layout (location = 1) in vec2 aTexCoord;

out mediump vec3 VertexColor;
out mediump vec2 TexCoord;

uniform vec3 WorldPosition;

void main() {
    gl_Position = vec4(ModelPosition + WorldPosition, 1.0);
    TexCoord = aTexCoord;
}