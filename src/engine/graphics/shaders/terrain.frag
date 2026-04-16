#version 430 core

uniform float ambientIntensity = 0.2;
uniform vec3 globalLightDir = vec3(-1, -1, -1);
uniform vec3 viewPos;

smooth in vec3 color;
in vec3 fragPos;

out vec4 outColor;

void main()
{

    // Calculate tangents using partial derivatives of the fragment position
    vec3 tangentX = dFdx(fragPos);
    vec3 tangentY = dFdy(fragPos);

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