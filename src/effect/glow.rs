use crate::color::Color;

pub struct GlowEffect {
    pub color: Color,
    pub radius: f32,
    pub intensity: f32,
}

impl GlowEffect {
    pub fn new(color: Color, radius: f32, intensity: f32) -> Self {
        Self { color, radius, intensity }
    }

    pub fn default_green() -> Self {
        Self {
            color: Color::GREEN,
            radius: 4.0,
            intensity: 0.8,
        }
    }

    pub fn get_color_vec(&self) -> [f32; 4] {
        [self.color.r, self.color.g, self.color.b, self.color.a]
    }
}
