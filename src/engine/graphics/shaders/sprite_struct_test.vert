#version 460

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

struct Sprite {
    vec3 position;
    vec4 dimensions;
    uint spriteID;
};

layout(set = 0, binding = 0) uniform sampler2D spriteSheet;

layout(set = 0, binding = 1) buffer InputData {
    mat4 view;
    mat4 projection;
    vec2 texelOffset;
};

layout(set = 0, std430, binding = 2) buffer spriteSSBO {
    int spriteCount;
    Sprite sprites[];
};

layout(set = 0, std430, binding=3) buffer spriteSheetSSBO {
    int spriteIDCount;
    vec4 spriteBounds[];
}; 

layout(location = 0) out vec2 texCoords;

void main()
{
    spriteCount = 69;
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

    view = mat4(
         1,  2,  3,  4,
         5,  6,  7,  8,
         9, 10, 11, 12,
        13, 14, 15, 16
    );

    projection = mat4(
        17, 18, 19, 20,
        21, 22, 23, 24,
        25, 26, 27, 28,
        29, 30, 31, 32
    );

    texelOffset = vec2(33, 34);

    spriteBounds[0] = vec4(1, 2, 3, 4);
    spriteBounds[1] = vec4(5, 6, 7, 8);

    spriteIDCount = 420;

    gl_Position = vec4(position, 1.0);
    texCoords = uv;
}