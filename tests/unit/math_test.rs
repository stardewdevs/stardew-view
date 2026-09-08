use stardew_view::utils::{MathUtils, MatrixUtils, VectorUtils};

#[test]
fn lerp_midpoint() {
    assert_eq!(MathUtils::lerp(0.0, 10.0, 0.5), 5.0);
}

#[test]
fn lerp_clamped_extremes() {
    assert_eq!(MathUtils::lerp(0.0, 10.0, 0.0), 0.0);
    assert_eq!(MathUtils::lerp(0.0, 10.0, 1.0), 10.0);
}

#[test]
fn lerp_reversed_inputs() {
    assert_eq!(MathUtils::lerp(10.0, 0.0, 0.5), 5.0);
}

#[test]
fn clamp_values() {
    assert_eq!(MathUtils::clamp(0.5, 0.0, 1.0), 0.5);
    assert_eq!(MathUtils::clamp(-0.5, 0.0, 1.0), 0.0);
    assert_eq!(MathUtils::clamp(1.5, 0.0, 1.0), 1.0);
}

#[test]
fn clamp_zero_range() {
    assert_eq!(MathUtils::clamp(100.0, 0.5, 0.5), 0.5);
}

#[test]
fn matrix_perspective_rh() {
    let m = MatrixUtils::perspective(60.0, 1.0, 0.1, 10.0);
    let near_row = m.z_axis.z;
    assert!(near_row > 0.0);
}

#[test]
fn matrix_orthographic_bounds() {
    let m = MatrixUtils::orthographic(0.0, 80.0, 24.0, 0.0, 0.0, 1.0);
    assert!(m.x_axis.x > 0.0);
}

#[test]
fn matrix_translation() {
    let m = MatrixUtils::translation(3.0, 4.0, 5.0);
    let p = m.transform_point3(VectorUtils::vec3(0.0, 0.0, 0.0));
    assert_eq!(p.x, 3.0);
    assert_eq!(p.y, 4.0);
    assert_eq!(p.z, 5.0);
}

#[test]
fn vector_lerp() {
    let v = VectorUtils::lerp_vec2(VectorUtils::vec2(0.0, 0.0), VectorUtils::vec2(4.0, 8.0), 0.5);
    assert_eq!(v.x, 2.0);
    assert_eq!(v.y, 4.0);
}

#[test]
fn vector_distance_3_4_5() {
    let d = VectorUtils::distance_vec3(VectorUtils::vec3(0.0, 0.0, 0.0), VectorUtils::vec3(3.0, 4.0, 0.0));
    assert!((d - 5.0).abs() < 1e-6);
}