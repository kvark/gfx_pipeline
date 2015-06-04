#version 150 core

const vec3 c_LightPos = vec3(10.0, 10.0, 10.0); //view space
uniform vec4 u_Color;
uniform vec4 u_Ambient;

in vec3 v_Normal;

out vec4 o_Color;

void main() {
	vec3 N = normalize(v_Normal);
	vec3 L = normalize(c_LightPos);
	float k_diffuse = max(0.0, dot(N, L));
	vec3 diffuse_color = u_Color.xyz * (u_Ambient.xyz + k_diffuse);
	o_Color = vec4(diffuse_color, u_Color.a);
}
