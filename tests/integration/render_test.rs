use stardew_view::buffer::VertexBuffer;
use stardew_view::color::{Color, Palette};
use stardew_view::renderer::Renderer;

#[test]
fn renderer_default_construction() {
    let renderer = Renderer::default();
    assert_eq!(renderer.width(), 0);
    assert_eq!(renderer.height(), 0);
}

#[test]
fn renderer_tick_does_not_panic() {
    let mut renderer = Renderer::default();
    renderer.tick();
    renderer.tick();
}

#[test]
fn renderer_clear_color_changes() {
    let mut renderer = Renderer::default();
    let original = renderer.clear_color();
    renderer.set_clear_color(Color::GREEN);
    assert_ne!(renderer.clear_color(), original);
}

#[test]
fn palette_default_has_green_foreground() {
    let palette = Palette::default();
    assert_eq!(palette.foreground, Color::GREEN);
    assert_eq!(palette.background, Color::BLACK);
}

#[test]
fn vertex_buffer_layout_matches_expected() {
    let layout = VertexBuffer::layout();
    let strides: Vec<u64> = layout.iter().map(|attr| attr.offset).collect();
    assert!(strides.len() >= 2);
}