use crossfont::Rasterizer;

pub struct FontManager {
    rasterizer: Rasterizer,
}

impl FontManager {
    pub fn new() -> Option<Self> {
        let rasterizer = Rasterizer::new(16.0, false).ok()?;
        Some(Self { rasterizer })
    }

    pub fn rasterizer(&mut self) -> &mut Rasterizer {
        &mut self.rasterizer
    }
}
