#version 450

layout(location = 0) in vec2 out_tex_coords;
layout(location = 1) in vec2 out_shadow_offset;

layout(set = 0, binding = 0) uniform sampler2D input_texture;

layout(push_constant) uniform ShadowParams {
    vec2 offset;
    vec2 texel_size;
    float blur_radius;
    vec4 color;
} params;

layout(location = 0) out vec4 frag_color;

void main() {
    vec2 shadow_coords = out_tex_coords - params.offset;
    float shadow_alpha = 0.0;
    int kernel_radius = int(params.blur_radius);

    for (int y = -kernel_radius; y <= kernel_radius; y++) {
        for (int x = -kernel_radius; x <= kernel_radius; x++) {
            vec2 offset = vec2(float(x) * params.texel_size.x, float(y) * params.texel_size.y);
            float weight = exp(-float(x * x + y * y) / (2.0 * params.blur_radius * params.blur_radius));
            vec4 sample = texture(input_texture, shadow_coords + offset);
            shadow_alpha += sample.a * weight;
            total += weight;
        }
    }
}