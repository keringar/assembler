#version 150 core

in vec2 v_TexCoord;
in vec4 v_Color;

out vec4 o_Color;

uniform sampler2D t_Color;

void main() {
    o_Color = v_Color;
}