use criterion::{black_box, criterion_group, criterion_main, Criterion};

use stardew_view::utils::{FpsCounter, MathUtils, Timer};

fn bench_fps_throughput(c: &mut Criterion) {
    c.bench_function("fps_throughput_10k_frames", |b| {
        b.iter(|| {
            let mut fps = FpsCounter::new();
            for _ in 0..10_000 {
                black_box(fps.tick());
            }
        })
    });
    c.bench_function("fps_clamp_rate", |b| {
        let mut fps = FpsCounter::new();
        b.iter(|| black_box(fps.tick()))
    });
}

fn bench_math_throughput(c: &mut Criterion) {
    c.bench_function("lerp_aggregate", |b| {
        b.iter(|| {
            let mut acc = 0.0f32;
            for i in 0..1000 {
                acc = black_box(MathUtils::lerp(acc, i as f32, 0.01));
            }
            acc
        })
    });
}

fn bench_timer_drift(c: &mut Criterion) {
    c.bench_function("timer_elapsed_drift", |b| {
        let mut timer = Timer::new();
        b.iter(|| {
            black_box(timer.elapsed());
            timer.reset();
        })
    });
}

criterion_group!(fps_benches, bench_fps_throughput, bench_math_throughput, bench_timer_drift);
criterion_main!(fps_benches);