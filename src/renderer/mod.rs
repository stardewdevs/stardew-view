use stardew_emulator::TerminalSnapshot;
use wgpu::{CommandEncoder, Device, Queue, TextureView};

pub struct Renderer;

impl Renderer {
    pub fn new(_device: &Device, _queue: &Queue) -> Self {
        Self
    }

    pub fn render(&mut self, _snapshot: &TerminalSnapshot) {}

    pub fn draw(&mut self, _encoder: &mut CommandEncoder, _view: &TextureView) {}
}
