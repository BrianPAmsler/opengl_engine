#version 460 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

uniform mat4 VP;

struct Sprite {
    vec3 position;
    vec4 dimensions;
    uint spriteID;
};

layout(std430, binding=2) buffer spriteSSBO {
    int spriteCount;
    Sprite sprites[];
};

layout(std430, binding=3) buffer spriteSheetSSBO {
    int spriteIDCount;
    vec4 spriteBounds[];
};

out vec2 texCoords;

void main()
{
    sprites[0] = Sprite(
        vec3(1, 2, 3),
        vec4(4, 5, 6, 7),
        8
    );

    sprites[1] = Sprite(
        vec3(9, 10, 11),
        vec4(12, 13, 14, 15),
        16
    );

    spriteBounds[0] = ivec4(1, 2, 3, 4);
    spriteBounds[1] = ivec4(5, 6, 7, 8);

    gl_Position = vec4(position, 1.0);
    texCoords = uv;
}