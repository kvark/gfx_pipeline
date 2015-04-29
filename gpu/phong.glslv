#version 150 core

uniform mat4 u_Transform;
uniform mat3 u_NormalRotation;

in vec3 a_Position;
in vec3 a_Normal;

out vec3 v_Normal;

void main() {
	gl_Position = u_Transform * vec4(a_Position, 1.0);
	v_Normal = u_NormalRotation * a_Normal;
}
