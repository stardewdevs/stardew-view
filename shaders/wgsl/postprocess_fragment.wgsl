@group(0) @binding(0) var sample_tex: texture_2d<f32>;
@group(0) @binding(1) var sample_sampler: sampler;

@fragment
fn main(
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
) -> @location(0) vec4<f32> {
    return textureSample(sample_tex, sample_sampler, tex_coords);
}