#version 150 core

const int MAX_LIGHTS = 256;

struct Light {
	vec4 position;
	vec4 color;
	vec4 attenuation;
};

uniform b_Lights {
	Light u_Lights[MAX_LIGHTS];
};

uniform uvec4 u_LightMask;
uniform sampler2D t_Diffuse;
uniform vec4 u_Color;
uniform vec4 u_Ambient;
uniform vec4 u_Attenuation;
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
	vec3 diffuse_color = u_Ambient.xyz;
	uvec4 light_mask = u_LightMask;

	while (light_mask != uvec4(0,0,0,0)) {
		Light light = u_Lights[light_mask.x & 0xFFU];
		// evaluate diffuse contribution
		vec3 L = normalize(light.position.xyz - v_Position * light.position.w);
		float k_diffuse = max(0.0, dot(N, L));
		// attenuation (TODO)
		float dist = length(light.position.xyz - v_Position);
		float attenuation = 1.0 / dot(u_Attenuation.xyz, vec3(1.0, dist, dist*dist));
		float k_attenu = mix(1.0, attenuation, light.position.w);
		// contribute
		diffuse_color += k_attenu * k_diffuse * light.color.xyz;
		// rotate the light mask
		light_mask = (light_mask >> 8) | uvec4(light_mask.yzw<<24, 0);
	}

	o_Color = vec4(diffuse_color * mat_color.xyz, mat_color.w);
}
