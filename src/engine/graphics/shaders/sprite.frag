#version 430 core

layout(location = 0) in vec2 texCoords;
layout(binding = 0) uniform sampler2D spriteSheet;

layout(location = 0) out vec4 outColor;

void main()
{
    outColor = texture(spriteSheet, texCoords);

    if (outColor.a < 0.1)
        discard;
}