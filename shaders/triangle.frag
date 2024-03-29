#version 450 core

layout (location = 0) in vec3 in_color;
layout (location = 1) in vec2 in_tex_coords;

uniform sampler2D tex;

layout(location = 0) out vec4 out_color;

void main()
{
    out_color = texture(tex, in_tex_coords);
}
