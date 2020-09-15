#version 330 core

in vec4 color;
in vec2 textur;


out vec4 rcolor;

uniform sampler2D sp;

void main()
{
    rcolor = texture(sp, textur) * color;
}