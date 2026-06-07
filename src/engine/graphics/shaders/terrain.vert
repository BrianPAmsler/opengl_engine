#version 430 core

layout(location = 0) in vec3 position;

layout(set = 0, binding = 0) uniform VertexUniforms {
    mat4 vp;
    uvec2 terrainDimensions;
    float heightScale;
};

layout(binding = 2) uniform sampler2D heightTex;
layout(binding = 3) uniform sampler2D colorTex;

layout(location = 0) smooth out vec2 uv;
layout(location = 1) out vec3 fragPos;
layout(location = 2) flat out vec3 colors[4];

vec2 colorFromIndex(uvec2 index, uvec2 corner) {
    vec2 uv = vec2(index) / vec2(terrainDimensions) + 0.25 / vec2(terrainDimensions) + vec2(corner) * (0.5 / vec2(terrainDimensions));

    // Flip v since uv is top-left and terrain coordinates are bottom-left
    return vec2(uv.x, 1 - uv.y);
}

vec2 heightFromIndex(uvec2 index) {
    vec2 dim = vec2(terrainDimensions + uvec2(1));
    vec2 uv =  vec2(index) / dim + 0.5 / dim;

    // Flip v since uv is top-left and terrain coordinates are bottom-left
    return vec2(uv.x, 1 - uv.y);
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

// (bottom-left, bottom-right, top-left, top-right, center)
const vec2 uvs[5] = {
    vec2(0, 1),
    vec2(1, 1),
    vec2(0, 0),
    vec2(1, 0),
    vec2(0.5, 0.5)
};

void main()
{
    vec3 outPosition = position;
    uvec2 cellIndex = uvec2(gl_InstanceIndex % terrainDimensions.x, gl_InstanceIndex / terrainDimensions.x);

    // Offset vertex by its x, y coords calculated from gl_InstanceID
    outPosition += vec3(cellIndex.x, 0, cellIndex.y);

    colors[0] = texture(colorTex, colorFromIndex(cellIndex, uvec2(0))).rgb;    // bottom_left
    colors[1] = texture(colorTex, colorFromIndex(cellIndex, uvec2(1, 0))).rgb; // bottom_right
    colors[2] = texture(colorTex, colorFromIndex(cellIndex, uvec2(0, 1))).rgb; // top_left
    colors[3] = texture(colorTex, colorFromIndex(cellIndex, uvec2(1, 1))).rgb; // top_right

    if (gl_VertexIndex == 4) {
        // All corners
        float a = texture(heightTex, heightFromIndex(cellIndex)).r;
        float b = texture(heightTex, heightFromIndex(cellIndex + uvec2(1, 0))).r;
        float c = texture(heightTex, heightFromIndex(cellIndex + uvec2(0, 1))).r;
        float d = texture(heightTex, heightFromIndex(cellIndex + uvec2(1, 1))).r;

        float medianHeight = median(a, b, c, d) * heightScale;

        outPosition += vec3(0, medianHeight, 0);
    } else { 
        // Offset index based on the corner
        uvec2 offset = offsets[gl_VertexIndex];

        float height = texture(heightTex, heightFromIndex(cellIndex + offset)).r * heightScale;

        outPosition += vec3(0, height, 0);
    }

    gl_Position = vp * vec4(outPosition, 1);
    gl_Position.z = (gl_Position.z + gl_Position.w) * 0.5;
    
    fragPos = outPosition;
    uv = uvs[gl_VertexIndex];
}