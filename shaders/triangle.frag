#version 450 core

layout (location = 0) in vec4 in_color;
layout (location = 1) in vec2 in_tex_coords;
layout (location = 2) flat in int in_tex_offset; // flat means not interpolated

layout (location = 0) uniform sampler2DArray tex;

layout(location = 0, index = 0) out vec4 out_color;
layout(location = 0, index = 1) out vec4 out_color_mask;

void main()
{
    out_color = in_color;
    out_color_mask = texture(tex, vec3(in_tex_coords, in_tex_offset)) * in_color.a;
}
