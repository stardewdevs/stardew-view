use alacritty_terminal::Term;
use sugarloaf::Sugarloaf;
use wgpu::{CommandEncoder, Device, TextureView};

pub struct Renderer {
    sugarloaf: Sugarloaf,
}

impl Renderer {
    pub fn new(device: Device) -> Self {
        let sugarloaf = Sugarloaf::new(&device);
        Self { sugarloaf }
    }

    pub fn render(&mut self, terminal: &Term) {
        let grid = terminal.grid();
        let cursor = terminal.cursor();
        let colors = terminal.colors();
        self.sugarloaf.render_grid(grid, cursor, colors);
    }

    pub fn draw(&mut self, encoder: &mut CommandEncoder, view: &TextureView) {
        self.sugarloaf.draw(encoder, view);
    }
}
