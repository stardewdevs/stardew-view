use criterion::{black_box, criterion_group, criterion_main, Criterion};

use stardew_view::buffer::VertexBuffer;
use stardew_view::color::{Color, Palette};
use stardew_view::effect::{BlurEffect, GlowEffect, ShadowEffect};
use stardew_view::renderer::Renderer;
use stardew_view::utils::{FpsCounter, MathUtils, Timer};

fn bench_math(c: &mut Criterion) {
    c.bench_function("math_lerp", |b| {
        b.iter(|| black_box(MathUtils::lerp(1.0, 2.0, black_box(0.5))))
    });
    c.bench_function("math_clamp", |b| {
        b.iter(|| black_box(MathUtils::clamp(black_box(3.0), 0.0, 1.0)))
    });
}

fn bench_color(c: &mut Criterion) {
    let color = Color::GREEN;
    c.bench_function("color_to_vec4", |b| b.iter(|| black_box(color.to_vec4())));
    let palette = Palette::default();
    c.bench_function("palette_to_json", |b| {
        b.iter(|| black_box(palette.to_json()))
    });
}

fn bench_effects(c: &mut Criterion) {
    let blur = BlurEffect::new(8.0, 0.9);
    c.bench_function("blur_kernel_gaussian", |b| {
        b.iter(|| black_box(blur.apply()))
    });
    c.bench_function("blur_kernel_size", |b| {
        b.iter(|| black_box(blur.get_kernel_size()))
    });

    let glow = GlowEffect::default_green();
    c.bench_function("glow_color", |b| b.iter(|| black_box(glow.get_color_vec())));

    let shadow = ShadowEffect::default_drop();
    c.bench_function("shadow_bounds", |b| {
        b.iter(|| black_box(shadow.get_effective_bounds(0.0, 0.0, 10.0, 10.0)))
    });
}

fn bench_fps(c: &mut Criterion) {
    c.bench_function("fps_counter_tick", |b| {
        let mut fps = FpsCounter::new();
        b.iter(|| black_box(fps.tick()))
    });
    c.bench_function("timer_elapsed", |b| {
        let mut timer = Timer::new();
        b.iter(|| black_box(timer.elapsed()))
    });
}

fn bench_renderer(c: &mut Criterion) {
    c.bench_function("renderer_update_state", |b| {
        let mut renderer = Renderer::default();
        b.iter(|| renderer.tick())
    });
    c.bench_function("vertex_buffer_layout", |b| {
        b.iter(|| black_box(VertexBuffer::layout()))
    });
}

criterion_group!(
    benches,
    bench_math,
    bench_color,
    bench_effects,
    bench_fps,
    bench_renderer
);
criterion_main!(benches);
