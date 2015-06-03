#version 150 core

uniform mat4 u_Transform;

in vec3 a_Position;
in vec2 a_Tex0;

out vec2 v_TexCoords;

void main() {
	gl_Position = u_Transform * vec4(a_Position, 1.0);
	v_TexCoords = a_Tex0;
}
