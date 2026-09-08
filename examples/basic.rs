use stardew_view::color::{Color, Palette};
use stardew_view::renderer::Renderer;

fn main() {
    let palette = Palette {
        foreground: Color::GREEN,
        background: Color::BLACK,
        cursor: Color::GREEN,
        selection: Color::new(0.15, 0.31, 0.47, 1.0),
    };

    println!("Palette foreground: {:?}", palette.foreground.to_vec4());

    let grid = alacritty_terminal::grid::Grid::new(
        alacritty_terminal::grid::Dimensions {
            columns: 80,
            lines: 24,
        },
        alacritty_terminal::term::cell::Cell::default(),
    );
    let cursor = alacritty_terminal::term::Point::new(2, 4);

    let renderer = Renderer::default();
    let _ = renderer.draw_grid(&grid, &cursor, &palette);
    println!(
        "sample: grid {}x{} at cursor ({}, {})",
        80, 24, cursor.line, cursor.column
    );
}
