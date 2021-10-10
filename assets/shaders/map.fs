#version 100

precision mediump float;

// Input vertex attributes (from vertex shader)
varying vec2 fragTexCoord;
varying vec4 fragColor;
varying vec3 fragPosition;
varying vec3 fragNormal;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;

void main()
{
    vec4 texelColor = texture2D(texture0, fragTexCoord);
    vec4 ambient = vec4(1.0,1.0,1.0,1.0);

    vec3 normal = normalize(fragNormal);

    vec3 lightPosition = vec3(1.0,50.0,1.0);
    vec3 lightTarget = vec3(0.0,0.0,0.0);

    vec3 lightDir = normalize(lightPosition - fragPosition);
    float NdotL = max(dot(normal, lightDir), 0.0);

    vec3 lightDot = vec3(1.0,1.0,1.0) * NdotL;

    gl_FragColor = texelColor*colDiffuse*fragColor*(vec4(lightDot, 1.0) + (ambient/10.0));
    /* gl_FragColor = vec4(fragNormal,1.0); */
}
