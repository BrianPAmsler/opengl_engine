#version 330 core

in vec2 texCoords;

uniform sampler2D spriteSheet;

out vec4 outColor;

void main()
{
    // Offset by half a texel to avoid artifacts caused by floating-point errors
    vec2 tex_offset = 0.5 / textureSize(spriteSheet, 0);

    outColor = texture(spriteSheet, texCoords + tex_offset);
}