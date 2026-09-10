use glyphon::{FontSystem, SwashCache};

pub struct FontManager {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl FontManager {
    pub fn new() -> Self {
        let mut font_system = FontSystem::new();
        #[cfg(target_os = "android")]
        font_system.db_mut().load_fonts_dir("/system/fonts");
        Self {
            font_system,
            swash_cache: SwashCache::new(),
        }
    }

    pub fn font_system_mut(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }

    pub fn swash_cache_mut(&mut self) -> &mut SwashCache {
        &mut self.swash_cache
    }
}
