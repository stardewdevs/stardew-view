use stardew_view::color::{Color, Palette};
use stardew_view::shader::{ShaderKind, ShaderManager};
use stardew_view::texture::TextureManager;

const VERTEX_SRC: &str = include_str!("../shaders/wgsl/terminal_vertex.wgsl");

fn main() {
    let shader = ShaderManager::compile_source(VERTEX_SRC, ShaderKind::Vertex);
    println!("compiled shader module: {:?}", shader);

    let texture = TextureManager::create_atlas(512, 512);
    println!("created atlas: {}x{}", texture.width, texture.height);

    let palette = Palette {
        foreground: Color::GREEN,
        background: Color::BLACK,
        cursor: Color::GREEN,
        selection: Color::new(0.15, 0.31, 0.47, 1.0),
    };
    println!("ready to render with palette {:?}", palette.to_json());
}