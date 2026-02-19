#version 430 core

uniform float ambientIntensity = 0.1;
uniform float specularStrength = 0.5;
uniform float shininess = 32;
uniform vec3 lightPos = vec3(-1, 1, -1);
uniform vec3 viewPos;

in vec3 color;
in vec3 fragPos;

out vec4 outColor;

void main()
{

    // Calculate tangents using partial derivatives of the fragment position
    vec3 tangentX = dFdx(fragPos);
    vec3 tangentY = dFdy(fragPos);

    // The cross product of the tangents gives the surface normal
    vec3 normal = normalize(cross(tangentX, tangentY));

    // Diffuse
    vec3 lightDir = normalize(lightPos - fragPos);  

    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * color;

    // Ambient
    vec3 ambient = vec3(ambientIntensity);

    // Specular
    vec3 viewDir = normalize(viewPos - fragPos);
    vec3 reflectDir = reflect(-lightDir, normal);

    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = vec3(specularStrength * spec);  

    vec3 final = diffuse + ambient + specular;

    outColor = vec4(final, 1);
}