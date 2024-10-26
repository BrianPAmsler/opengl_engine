#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

uniform mat4 VP;

struct Sprite {
    vec3 position;
    vec4 dimensions;
    vec4 uvs;
    uint spriteID;
};

layout(std430, binding = 2) buffer spriteSSBO {
    int spriteCount;
    Sprite sprites[];
};

out vec2 texCoords;

void main()
{
    Sprite sprite = sprites[gl_InstanceID];

    vec2 anchor = sprite.dimensions.xy;
    vec2 scale = sprite.dimensions.zw;

    mat4 model = mat4(1);
    // model[3] = vec4(sprite.position - sprite.anchor * sprite.dimensions, 1);

    mat4 mvp = model * VP;

    gl_Position = vec4(position, 1.0) * mvp;
    texCoords = uv;
}