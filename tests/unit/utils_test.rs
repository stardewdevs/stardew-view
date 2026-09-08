use std::time::Duration;

use stardew_view::utils::{FpsCounter, Timer};

#[test]
fn fps_counter_starts_at_zero() {
    let fps = FpsCounter::new();
    assert_eq!(fps.frame_count, 0);
}

#[test]
fn fps_counter_tick_increments() {
    let mut fps = FpsCounter::new();
    fps.tick();
    fps.tick();
    assert_eq!(fps.frame_count, 2);
}

#[test]
fn fps_counter_reports_zero_early() {
    let mut fps = FpsCounter::new();
    assert_eq!(fps.tick(), 0.0);
}

#[test]
fn timer_elapsed_non_zero_after_wait() {
    let mut timer = Timer::new();
    std::thread::sleep(Duration::from_millis(5));
    assert!(timer.elapsed() >= Duration::from_millis(5));
}

#[test]
fn timer_reset_returns_to_zero() {
    let mut timer = Timer::new();
    std::thread::sleep(Duration::from_millis(2));
    timer.reset();
    assert!(timer.elapsed() < Duration::from_millis(2));
}

#[test]
fn timer_duration_measurement() {
    let mut timer = Timer::new();
    let before = std::time::Instant::now();
    std::thread::sleep(Duration::from_millis(10));
    let elapsed = timer.elapsed();
    let after = before.elapsed();
    assert!(elapsed <= after + Duration::from_millis(5));
}