use criterion::{black_box, criterion_group, criterion_main, Criterion};

use stardew_view::utils::{MatrixUtils, VectorUtils};

fn bench_matrix(c: &mut Criterion) {
    c.bench_function("matrix_perspective", |b| {
        b.iter(|| black_box(MatrixUtils::perspective(60.0, 1.3333, 0.1, 100.0)))
    });
    c.bench_function("matrix_orthographic", |b| {
        b.iter(|| black_box(MatrixUtils::orthographic(0.0, 80.0, 0.0, 24.0, 0.0, 1.0)))
    });
    c.bench_function("matrix_look_at", |b| {
        b.iter(|| {
            black_box(MatrixUtils::look_at(
                VectorUtils::vec3(0.0, 0.0, 5.0),
                VectorUtils::vec3(0.0, 0.0, 0.0),
                VectorUtils::vec3(0.0, 1.0, 0.0),
            ))
        })
    });
    c.bench_function("matrix_rotation", |b| {
        b.iter(|| black_box(MatrixUtils::rotation_z(black_box(0.5))))
    });
}

fn bench_vector(c: &mut Criterion) {
    c.bench_function("vector_lerp", |b| {
        b.iter(|| {
            black_box(VectorUtils::lerp_vec2(
                VectorUtils::vec2(0.0, 0.0),
                VectorUtils::vec2(10.0, 10.0),
                black_box(0.5),
            ))
        })
    });
    c.bench_function("vector_distance", |b| {
        b.iter(|| {
            black_box(VectorUtils::distance_vec3(
                VectorUtils::vec3(0.0, 0.0, 0.0),
                VectorUtils::vec3(3.0, 4.0, 0.0),
            ))
        })
    });
}

criterion_group!(matrix_benches, bench_matrix, bench_vector);
criterion_main!(matrix_benches);
