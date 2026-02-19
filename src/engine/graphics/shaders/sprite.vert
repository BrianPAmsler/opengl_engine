#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

uniform mat4 view;
uniform mat4 projection;
uniform vec2 texelOffset;

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

layout(std430, binding=4) buffer debugSSBO {
    vec2 padding;
    vec2 tex_offset_debug;
    vec2 tex_size;
};

out vec2 texCoords;
flat out int spriteID;

void main()
{
    Sprite sprite = sprites[gl_InstanceID];

    vec2 anchor = sprite.dimensions.xy;
    vec2 scale = sprite.dimensions.zw;

    // Map 0 to 1 and 1 to -1.
    // Offset direction based on the position of the mesh vertex.
    // This makes sure the half-texel offset stays inside the bounds of the texture.
    // The half-texel offset prevents texture bleed at the edges of a sprtite due to floating point precision issues.
    // This is inteded for a square mesh with vertices (0, 0), (0, 1), (1, 0), (1, 1).
    // Any other mesh will have unexpected resutls
    vec2 offsetDirection = -((position.xy - 0.5) * 2);

    vec3 translation = sprite.position;//- vec3(anchor * scale, 0);

    mat4 inverseView = mat4(inverse(mat3(view)));

    mat4 model = transpose(mat4(
        scale.x,    0,          0,  translation.x,
        0,          scale.y,    0,  translation.y,
        0,          0,          1,  translation.z,
        0,          0,          0,  1
    ));

    // model = inverseView * model;

    gl_Position = projection * view * model * vec4(position, 1);

    vec4 bounds = spriteBounds[sprite.spriteID];
    texCoords = bounds.xy + uv * bounds.zw + texelOffset * offsetDirection;
}