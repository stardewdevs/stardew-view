use wgpu::{Surface, Instance};

pub struct WindowsSurface {
    pub surface: Surface,
    pub width: u32,
    pub height: u32,
}

impl WindowsSurface {
    pub fn new(surface: Surface, width: u32, height: u32) -> Self {
        Self { surface, width, height }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}