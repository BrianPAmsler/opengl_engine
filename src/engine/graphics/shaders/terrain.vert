#version 430 core

layout(location = 0) in vec3 position;

uniform mat4 vp;
uniform uvec2 terrainDimensions;
uniform float heightScale;

layout(binding = 0) uniform sampler2D heightTex;
layout(binding = 1) uniform sampler2D colorTex;

out vec3 color;
out vec3 fragPos;

vec2 uvFromIndex(uvec2 index) {
    return vec2(index) / vec2(terrainDimensions);
}

float median(float a, float b, float c, float d) {
    float arr[3] = { b, c, d };

    float total = a;
    float min = a;
    float max = a;

    // This is probably slower than just sorting, but it looks cool
    for (int i = 0; i < 3; i ++) {
        total += arr[i];
        if (arr[i] > max)
            max = arr[i];

        if (arr[i] < min)
            min = arr[i];
    }

    // subtract off the max and the min so we are left with just the middle two
    return (total - min - max) / 2;
}

// Vertex Indices
// [0]: Bottom-Left Corner      offset: (0, 0)
// [1]: Bottom-Right Corner     offset: (1, 0)
// [2]: Top-Left Corner         offset: (0, 1)
// [3]: Top-Right Corner        offset: (1, 1)
// [4]: Center                  no offset (height is calculated from all 4 corners)
const uvec2 offsets[4] = {
    uvec2(0, 0),
    uvec2(1, 0),
    uvec2(0, 1),
    uvec2(1, 1)
};

void main()
{
    vec3 outPosition = position;
    uvec2 cellIndex = uvec2(gl_InstanceID % terrainDimensions.x, gl_InstanceID / terrainDimensions.x);

    // Offset vertex by its x, y coords calculated from gl_InstanceID
    outPosition += vec3(cellIndex.x, 0, cellIndex.y);

    if (gl_VertexID == 4) {
        // All corners
        float a = texture(heightTex, uvFromIndex(cellIndex)).r;
        float b = texture(heightTex, uvFromIndex(cellIndex + uvec2(1, 0))).r;
        float c = texture(heightTex, uvFromIndex(cellIndex + uvec2(0, 1))).r;
        float d = texture(heightTex, uvFromIndex(cellIndex + uvec2(1, 1))).r;

        float medianHeight = median(a, b, c, d) * heightScale;

        outPosition += vec3(0, medianHeight, 0);
    } else {
        // Offset index based on the corner
        uvec2 offset = offsets[gl_VertexID];

        float height = texture(heightTex, uvFromIndex(cellIndex + offset)).r * heightScale;

        outPosition += vec3(0, height, 0);
    }

    gl_Position = vp * vec4(outPosition, 1);
    fragPos = outPosition;
}