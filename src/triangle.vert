#version 330 core

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 tex;
layout (location = 2) in vec4 col;

out vec2 textur;
out vec4 color;

uniform mat4 transform;
uniform vec2 viewsize;

void main()
{
    gl_Position = transform * vec4(pos/viewsize, 0.0, 1.0);
    color = col;
    textur = tex;
}