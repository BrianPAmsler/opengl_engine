#version 430 core

layout(binding = 0) uniform sampler2D _a;
layout(binding = 1) uniform sampler2D _b;
layout(binding = 2) uniform sampler2D noiseMap;

uniform float ambientIntensity = 0.2;
uniform vec3 globalLightDir = vec3(-1, -1, -1);
uniform vec3 viewPos;
uniform float pixelSize = 0.02;
uniform int noiseMapSize = 1;

smooth in vec2 uv;
in vec3 fragPos;
flat in vec3 colors[4];

out vec4 outColor;

void main()
{
    // Calculate tangents using partial derivatives of the fragment position
    vec3 tangentX = dFdx(fragPos);
    vec3 tangentY = dFdy(fragPos);

    // [0]: Bottom-Left Corner      offset: (0, 0)
    // [1]: Bottom-Right Corner     offset: (1, 0)
    // [2]: Top-Left Corner         offset: (0, 1)
    // [3]: Top-Right Corner        offset: (1, 1)

    vec2 clamped_uv = vec2(ivec2(uv / pixelSize) * pixelSize);
    float vertex_weight[4] = {
        (1 - clamped_uv.x) * (1 - clamped_uv.y),
        clamped_uv.x * (1 - clamped_uv.y),
        (1 - clamped_uv.x) * clamped_uv.y,
        clamped_uv.x * clamped_uv.y
    };
    float randomNumber = texture(noiseMap, fragPos.xz / float(noiseMapSize) / pixelSize).r;
    float t = randomNumber;
    float noise = (randomNumber - 0.5) * 0.05;
    // vec3 color = vec3(0);
    // colors[0] * vertex_weight.x + colors[1] * vertex_weight.y + colors[2] * vertex_weight.z + colors[3] * vertex_weight.w;
    int i = 0;
    while (i < 3 && randomNumber > vertex_weight[i]) {
        randomNumber -= vertex_weight[i];
        i++;
    }
    vec3 color = colors[i];

    // The cross product of the tangents gives the surface normal
    vec3 normal = normalize(cross(tangentY, tangentX));

    // Diffuse
    vec3 lightDir = normalize(-globalLightDir);  

    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * color;

    // Ambient
    vec3 ambient = vec3(ambientIntensity) * color;

    vec3 final = diffuse + ambient;

    outColor = vec4(final, 1);
}