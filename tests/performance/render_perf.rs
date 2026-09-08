use stardew_view::effect::BlurEffect;
use stardew_view::renderer::Renderer;

#[test]
fn renderer_tick_thousands_of_frames() {
    let mut renderer = Renderer::default();
    let start = std::time::Instant::now();
    for _ in 0..10_000 {
        renderer.tick();
    }
    let elapsed = start.elapsed();
    assert!(elapsed < std::time::Duration::from_secs(5));
}

#[test]
fn blur_kernel_large_radius_fast() {
    let blur = BlurEffect::new(32.0, 1.0);
    let start = std::time::Instant::now();
    let weights = blur.apply();
    let elapsed = start.elapsed();
    assert_eq!(weights.len(), 65);
    assert!(elapsed < std::time::Duration::from_secs(1));
}

#[test]
fn fps_counter_million_ticks() {
    let mut fps = stardew_view::utils::FpsCounter::new();
    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        fps.tick();
    }
    assert!(start.elapsed() < std::time::Duration::from_secs(5));
    assert_eq!(fps.frame_count, 1_000_000);
}