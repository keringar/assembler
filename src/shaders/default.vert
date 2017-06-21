#version 150 core

in vec2 a_Pos;
in vec3 a_Color;
in vec2 a_TexCoord;

out vec2 v_TexCoord;
out v_Color;

uniform Uniforms {
    mat4 u_Model;
    mat4 u_View;
    mat4 u_Proj;
};

void main() {
    v_TexCoord = a_TexCoord;
    v_Color = vec4(a_Color, 1.0);

    gl_Position = u_Model * u_View * u_Proj * vec4(a_Pos, 1.0);
}