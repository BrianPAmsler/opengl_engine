#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

uniform mat4 VP;

struct Sprite {
    vec3 position;
    vec4 dimensions;
    vec4 uvs;
    vec3 anchor;
    uint spriteID;
};

layout(std430, binding = 2) buffer spriteSSBO {
    int spriteCount;
    Sprite sprites[];
};

out vec2 texCoords;

void main()
{
    sprites[0].position = vec3(1, 2, 3);
    sprites[0].dimensions = vec4(4, 5, 6, 7);
    sprites[0].uvs = vec4(8, 9, 10, 11);
    sprites[0].anchor = vec3(12, 13, 14);
    sprites[0].spriteID = floatBitsToInt(15);

    sprites[1].position = vec3(16, 17, 18);
    sprites[1].position = vec3(16, 17, 18);
    sprites[1].dimensions = vec4(19, 20, 21, 22);
    sprites[1].uvs = vec4(23, 24, 25, 26);
    sprites[1].anchor = vec3(27, 28, 29);
    sprites[1].spriteID = floatBitsToInt(30);

    gl_Position = vec4(position, 1.0);
    texCoords = uv;
}