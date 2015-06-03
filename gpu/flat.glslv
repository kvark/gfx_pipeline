#version 150 core

uniform mat4 u_Transform;

in vec3 a_Position;

void main() {
	gl_Position = u_Transform * vec4(a_Position, 1.0);
}
