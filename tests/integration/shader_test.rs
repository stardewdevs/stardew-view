use stardew_view::shader::ShaderManager;

const TERMINAL_VERTEX: &str = include_str!("../../shaders/wgsl/terminal_vertex.wgsl");
const TERMINAL_FRAGMENT: &str = include_str!("../../shaders/wgsl/terminal_fragment.wgsl");
const POST_VERTEX: &str = include_str!("../../shaders/wgsl/postprocess_vertex.wgsl");
const POST_FRAGMENT: &str = include_str!("../../shaders/wgsl/postprocess_fragment.wgsl");

#[test]
fn shader_sources_have_entry_point() {
    for (name, src) in [
        ("terminal_vertex", TERMINAL_VERTEX),
        ("terminal_fragment", TERMINAL_FRAGMENT),
        ("postprocess_vertex", POST_VERTEX),
        ("postprocess_fragment", POST_FRAGMENT),
    ] {
        assert!(src.contains("fn main"), "{name} shader missing entry point");
        assert!(src.contains("@"), "{name} shader missing attributes");
    }
}

#[test]
fn shader_ud_channels_bindings_matched() {
    assert!(TERMINAL_FRAGMENT.contains("@group(0) @binding(0)"));
    assert!(TERMINAL_FRAGMENT.contains("@group(1) @binding(0)"));
    assert!(POST_FRAGMENT.contains("@group(0) @binding(0)"));
    assert!(POST_FRAGMENT.contains("@group(0) @binding(1)"));
}

#[test]
fn shader_compiler_accepts_sources() {
    let vertex = ShaderManager::compile_source(TERMINAL_VERTEX, stardew_view::shader::ShaderKind::Vertex);
    let fragment = ShaderManager::compile_source(TERMINAL_FRAGMENT, stardew_view::shader::ShaderKind::Fragment);
    assert!(vertex.is_ok());
    assert!(fragment.is_ok());
}

#[test]
fn shader_compiler_rejects_empty_source() {
    let vertex = ShaderManager::compile_source("", stardew_view::shader::ShaderKind::Vertex);
    assert!(vertex.is_err());
}