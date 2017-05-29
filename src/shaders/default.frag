#version 150 core

in vec2 v_TexCoord;
in vec4 v_Color;

out vec4 o_Color;

uniform sampler2D tex_color;

void main() {
    o_Color = v_Color;
}