#version 430 core

in vec2 texCoords;
layout(binding = 0) uniform sampler2D spriteSheet;

out vec4 outColor;

void main()
{
    outColor = texture(spriteSheet, texCoords);
}