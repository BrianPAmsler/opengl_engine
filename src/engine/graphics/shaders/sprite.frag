#version 430 core

in vec2 texCoords;

layout(binding = 0) uniform sampler2D spriteSheet;

out vec4 outColor;

void main()
{
    uint a = 1;
    outColor = texture(spriteSheet, texCoords);
}