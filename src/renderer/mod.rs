use wgpu::{CommandEncoder, Device, Queue, TextureView};
use alacritty_terminal::Term;

pub struct Renderer;

impl Renderer {
    pub fn new(_device: &Device, _queue: &Queue) -> Self {
        Self
    }

    pub fn render(&mut self, _terminal: &Term<()>) {}

    pub fn draw(&mut self, _encoder: &mut CommandEncoder, _view: &TextureView) {}
}
