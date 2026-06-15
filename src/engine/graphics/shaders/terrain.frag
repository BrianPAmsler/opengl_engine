#version 430 core

layout(binding = 4) uniform sampler2D noiseMap;

layout(set = 0, binding = 1) uniform FragmentUniforms {
    float ambientIntensity;
    vec3 globalLightDir;
    vec3 viewPos;
    float pixelSize;
    int noiseMapSize;
};

layout(location = 0) smooth in vec2 uv;
layout(location = 1) in vec3 fragPos;
layout(location = 2) flat in vec3 colors[4];

layout(location = 0) out vec4 outColor;

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
        (1 - clamped_uv.x) * clamped_uv.y,
        clamped_uv.x * clamped_uv.y,
        (1 - clamped_uv.x) * (1 - clamped_uv.y),
        clamped_uv.x * (1 - clamped_uv.y)
    };
    float randomNumber = texture(noiseMap, fragPos.xz / float(noiseMapSize) / pixelSize).r;
    float noise = (randomNumber - 0.5) * 0.05;
    int i = 0;
    while (i < 3 && randomNumber > vertex_weight[i]) {
        randomNumber -= vertex_weight[i];
        i++;
    }
    vec3 color = colors[i];

    // The cross product of the tangents gives the surface normal
    vec3 normal = normalize(cross(tangentY, tangentX));

    // Diffuse
    vec3 lightDir = normalize(globalLightDir);  

    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * color;

    // Ambient
    vec3 ambient = vec3(ambientIntensity) * color;

    vec3 final = diffuse + ambient;

    outColor = vec4(final + noise, 1);
}