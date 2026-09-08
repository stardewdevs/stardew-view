use crossfont::{Font, FontDesc};
use glyphon::FontSystem;

pub struct FontManager {
    pub system: FontSystem,
    pub default_font: FontDesc,
}

impl FontManager {
    pub fn new() -> Self {
        let mut system = FontSystem::new();
        let default_font = FontDesc::new("monospace", 16.0);
        Self {
            system,
            default_font,
        }
    }

    pub fn load(&mut self, desc: FontDesc) {
        self.system.add_font(desc);
    }

    pub fn get_font(&self, desc: &FontDesc) -> Option<Font> {
        self.system.get_font_by_desc(desc)
    }
}
