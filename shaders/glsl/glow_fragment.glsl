#version 450

layout(location = 0) in vec2 out_tex_coords;
layout(location = 1) in vec2 out_screen_pos;

layout(set = 0, binding = 0) uniform sampler2D input_texture;
layout(set = 0, binding = 1) uniform sampler2D glow_color;

layout(push_constant) uniform GlowParams {
    vec2 texel_size;
    float radius;
    float intensity;
    vec4 color;
} params;

layout(location = 0) out vec4 frag_color;

void main() {
    vec4 scene = texture(input_texture, out_tex_coords);
    vec4 glow = vec4(0.0);
    float total = 0.0;
    int kernel_radius = int(params.radius);

    for (int y = -kernel_radius; y <= kernel_radius; y++) {
        for (int x = -kernel_radius; x <= kernel_radius; x++) {
            vec2 offset = vec2(float(x) * params.texel_size.x, float(y) * params.texel_size.y);
            float weight = exp(-float(x * x + y * y) / (2.0 * params.radius * params.radius));
            glow += texture(input_texture, out_tex_coords + offset) * weight;
            total += weight;
        }
    }

    glow /= total;
    glow.rgb *= params.color.rgb;
    glow.a *= params.color.a * params.intensity;

    frag_color = scene + glow;
}