use crossfont::{Font, FontDesc, Glyph};
use glyphon::{FontSystem, SwashCache};

pub struct GlyphCache {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl GlyphCache {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        }
    }

    pub fn load_font(&mut self, desc: FontDesc) {
        self.font_system.add_font(desc);
    }

    pub fn rasterize(&mut self, glyph: &Glyph) -> Option<glyphon::Glyph> {
        let font = self.font_system.get_font(&glyph.font_id).unwrap();
        Some(glyphon::Glyph::new(font, glyph))
    }
}
