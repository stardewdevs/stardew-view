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

    pub fn load_font(&mut self, data: &[u8]) {
        self.font_system.db_mut().load_font_data(data.to_vec());
    }

    pub fn font_system_mut(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }

    pub fn swash_cache_mut(&mut self) -> &mut SwashCache {
        &mut self.swash_cache
    }
}
