#version 150 core

const int MAX_LIGHTS = 256;

struct Light {
	position: vec4,
	color: vec4,
	attenuation: vec4,
}

uniform b_Lights {
	Light u_Lights[MAX_LIGHTS];
};

uniform uint4 u_LightMask;
uniform sampler2D t_Diffuse;
uniform vec4 u_Color;
uniform vec4 u_Ambient;
uniform float u_AlphaTest;

in vec3 v_Position;
in vec3 v_Normal;
in vec2 v_TexCoords;

out vec4 o_Color;

void main() {
	vec4 tex = texture(t_Diffuse, v_TexCoords);
	vec4 mat_color = u_Color * tex;
	if (mat_color.w < u_AlphaTest)
		discard;

	vec3 N = normalize(v_Normal);
	vec3 diffuse_color = u_Ambient;

	while (u_LightMask != 0) {
		Light light = u_Lights[u_LightMask.x & 0xFF];
		// evaluate diffuse contribution
		vec3 L = normalize(light.pos.xyz - v_Position * light.pos.w);
		float k_diffuse = max(0.0, dot(N, L));
		diffuse_color += k_diffuse * light.color.xyz;
		// attenuation (TODO)
		// rotate the light mask
		u_LightMask = (u_LightMask >> 8) | float4(u_LightMask.yzw<<24, 0);
	}

	o_Color = vec4(diffuse_color * mat_color.xyz, mat_color.w);
}
