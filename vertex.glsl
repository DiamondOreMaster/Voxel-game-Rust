#version 460 core
layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec2 aTexCoords;

uniform mat4 u_Model;
uniform mat4 u_View;
uniform mat4 u_Projection;

out vec2 fTexCoords;

void main() {
    fTexCoords = aTexCoords;
    gl_Position = u_Projection * u_View * (u_Model * vec4(aPosition, 1.0));
}