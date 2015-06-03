#version 150 core

uniform sampler2D t_Diffuse;
uniform vec4 u_Color;
uniform float u_AlphaTest;

in vec2 v_TexCoords;

out vec4 o_Color;

void main() {
	vec4 tex = texture(t_Diffuse, v_TexCoords);
	vec4 color = u_Color * tex;
	if (color.a < u_AlphaTest)
		discard;
	o_Color = color;
}
