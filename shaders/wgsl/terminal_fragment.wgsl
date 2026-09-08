@group(0) @binding(0) var font_atlas: texture_2d<f32>;
@group(0) @binding(1) var font_sampler: sampler;

struct Uniforms {
    background: vec4<f32>,
    foreground: vec4<f32>,
    cursor: vec4<f32>,
}

@group(1) @binding(0) var<uniform> uniforms: Uniforms;

@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    let texel = textureSample(font_atlas, font_sampler, input.tex_coords);
    let color = mix(uniforms.background, uniforms.foreground, texel.r);
    return color;
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}