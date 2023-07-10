#version 460 core

out vec4 FragColor;

in vec2 fTexCoords;

uniform sampler2D uTexture;

void main()
{
    FragColor = texture(uTexture, fTexCoords);
}