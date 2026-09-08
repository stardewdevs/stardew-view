use stardew_view::color::{Color, Palette};

#[test]
fn color_constants() {
    assert_eq!(Color::BLACK.to_vec4(), [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(Color::WHITE.to_vec4(), [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(Color::GREEN.to_vec4(), [0.0, 1.0, 0.0, 1.0]);
}

#[test]
fn color_to_vec4_preserves_channels() {
    let c = Color::new(0.1, 0.2, 0.3, 0.4);
    assert_eq!(c.to_vec4(), [0.1, 0.2, 0.3, 0.4]);
}

#[test]
fn color_from_rgba_roundtrip() {
    let c = Color::from_rgba(255, 128, 0, 255);
    let v = c.to_vec4();
    assert_eq!(v, [1.0, 128.0 / 255.0, 0.0, 1.0]);
}

#[test]
fn palette_default_sanity() {
    let p = Palette {
        foreground: Color::GREEN,
        background: Color::BLACK,
        cursor: Color::GREEN,
        selection: Color::BLACK,
    };
    assert_eq!(p.foreground, p.cursor);
}

#[test]
fn palette_to_json_roundtrip() {
    let p = Palette::default();
    let json = p.to_json();
    assert!(json.contains("\"foreground\""));
    assert!(json.contains("\"background\""));
    assert!(json.contains("\"cursor\""));
    assert!(json.contains("\"selection\""));
}