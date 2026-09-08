#version 450

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec2 in_tex_coords;

layout(location = 0) out vec2 out_tex_coords;
layout(location = 1) out vec2 out_shadow_offset;

layout(push_constant) uniform ShadowParams {
    vec2 offset;
    vec2 texel_size;
    float blur_radius;
    vec4 color;
} params;

void main() {
    gl_Position = vec4(in_position, 0.0, 1.0);
    out_tex_coords = in_tex_coords;
    out_shadow_offset = params.offset;
}