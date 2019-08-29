#version 450 core

layout (location = 0) in vec3 in_color;
layout (location = 1) in vec2 in_tex_coords;

uniform sampler2D tex;

layout(location = 0, index = 0) out vec4 out_color;
layout(location = 0, index = 1) out vec4 out_color_mask;

void main()
{
    // TODO: pass alpha value too
    float alpha = 0.87;
    out_color = vec4(in_color,alpha);
    out_color_mask = texture(tex, in_tex_coords) * alpha;
}
