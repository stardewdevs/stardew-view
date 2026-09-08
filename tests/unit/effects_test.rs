use stardew_view::effect::{BlurEffect, GlowEffect, ShadowEffect};

#[test]
fn blur_kernel_weights_sum_to_one() {
    let blur = BlurEffect::new(4.0, 1.0);
    let weights = blur.apply();
    let sum: f32 = weights.iter().sum();
    assert!((sum - 1.0).abs() < 1e-3);
}

#[test]
fn blur_kernel_size() {
    let blur = BlurEffect::new(4.0, 1.0);
    assert_eq!(blur.get_kernel_size(), 9);
    let blur = BlurEffect::new(0.0, 1.0);
    assert_eq!(blur.get_kernel_size(), 1);
}

#[test]
fn blur_intensity_scales_center() {
    let low = BlurEffect::new(4.0, 0.5);
    let high = BlurEffect::new(4.0, 1.0);
    let low_weights = low.apply();
    let high_weights = high.apply();
    assert!(high_weights[high_weights.len() / 2] > low_weights[low_weights.len() / 2]);
}

#[test]
fn glow_default_green() {
    let glow = GlowEffect::default_green();
    assert_eq!(glow.get_color_vec(), [0.0, 1.0, 0.0, 0.8]);
    assert!(glow.radius > 0.0);
}

#[test]
fn glow_custom_color() {
    let glow = GlowEffect::new(GlowEffect::white(), 2.0, 1.0);
    assert_eq!(glow.get_color_vec(), [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn shadow_default_drop() {
    let shadow = ShadowEffect::default_drop();
    assert_eq!(shadow.offset, [2.0, 2.0]);
    assert_eq!(shadow.color, [0.0, 0.0, 0.0, 0.5]);
}

#[test]
fn shadow_bounds_expand() {
    let shadow = ShadowEffect::default_drop();
    let (x, y, w, h) = shadow.get_effective_bounds(0.0, 0.0, 10.0, 10.0);
    assert!(w > 10.0);
    assert!(h > 10.0);
    assert!(x > 0.0);
    assert!(y > 0.0);
}

#[test]
fn effect_manager_apply() {
    let mut manager = stardew_view::effect::EffectManager::default();
    manager.apply_blur();
    manager.apply_glow();
}