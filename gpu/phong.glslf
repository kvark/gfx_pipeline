#version 150 core

const vec3 c_LightPos = vec3(10.0, 10.0, 10.0); //view space
uniform sampler2D t_Diffuse;
uniform vec4 u_Color;

in vec3 v_Normal;
in vec2 v_TexCoords;

out vec4 o_Color;

void main() {
    vec4 tex = texture(t_Diffuse, v_TexCoords);
    vec3 N = normalize(v_Normal);
    vec3 L = normalize(c_LightPos);
    float k_diffuse = max(0.0, dot(N, L));
    vec3 diffuse_color = u_Color.xyz * k_diffuse;
    o_Color = vec4(diffuse_color, 1.0) * tex;
}
