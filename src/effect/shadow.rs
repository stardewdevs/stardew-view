use glam::Vec2;

pub struct ShadowEffect {
    pub offset: Vec2,
    pub blur_radius: f32,
    pub color: [f32; 4],
    pub spread: f32,
}

impl ShadowEffect {
    pub fn new(offset: Vec2, blur_radius: f32, color: [f32; 4], spread: f32) -> Self {
        Self { offset, blur_radius, color, spread }
    }

    pub fn default_drop() -> Self {
        Self {
            offset: Vec2::new(2.0, 2.0),
            blur_radius: 4.0,
            color: [0.0, 0.0, 0.0, 0.5],
            spread: 0.0,
        }
    }

    pub fn get_effective_bounds(&self, x: f32, y: f32, w: f32, h: f32) -> (f32, f32, f32, f32) {
        (
            x - self.spread + self.offset.x - self.blur_radius,
            y - self.spread + self.offset.y - self.blur_radius,
            w + self.spread * 2.0 + self.blur_radius * 2.0,
            h + self.spread * 2.0 + self.blur_radius * 2.0,
        )
    }
}
