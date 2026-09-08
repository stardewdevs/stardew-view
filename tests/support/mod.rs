pub use stardew_view::color::{Color, Palette};

pub fn test_palette() -> Palette {
    Palette {
        foreground: Color::GREEN,
        background: Color::BLACK,
        cursor: Color::GREEN,
        selection: Color::new(0.15, 0.31, 0.47, 1.0),
    }
}

pub fn assert_close(a: f32, b: f32, epsilon: f32) {
    assert!(
        (a - b).abs() <= epsilon,
        "expected {a} close to {b} within {epsilon}"
    );
}