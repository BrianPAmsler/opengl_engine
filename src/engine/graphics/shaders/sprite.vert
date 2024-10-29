#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

uniform mat4 view;
uniform mat4 projection;

struct Sprite {
    vec3 position;
    vec4 dimensions;
    uint spriteID;
};

layout(binding = 0) uniform sampler2D spriteSheet;

layout(std430, binding = 2) buffer spriteSSBO {
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
    Sprite sprite = sprites[gl_InstanceID];

    vec2 anchor = sprite.dimensions.xy;
    vec2 scale = sprite.dimensions.zw;

    vec4 center = vec4(sprite.position, 1) * view;

    vec3 translated = position - vec3(anchor, 0);
    vec3 scaled = translated * vec3(scale, 1);

    vec4 view_space = vec4(scaled, 0) + center;

    gl_Position = view_space * projection;

    // Offset by half a texel to avoid artifacts caused by floating-point errors
    vec2 tex_offset = 0.5 / textureSize(spriteSheet, 0);

    vec4 bounds = spriteBounds[sprite.spriteID] + vec4(tex_offset, -tex_offset);
    texCoords = bounds.xy + uv * bounds.zw;
}