use std::fmt;

#[derive(Debug)]
pub struct Glyph {
    pub character: char,
    pub width: usize,
    pub height: usize,
    pub bearing_x: i32,
    pub bearing_y: i32,
    pub advance: f32,
    pub bitmap: &'static [u8],
}

pub struct Font {
    pub size: u32,
    pub glyphs: &'static [Option<Glyph>],
}

impl Font {
    pub fn glyph(&self, character: char) -> Option<&Glyph> {
        self.glyphs
            .iter()
            .flatten()
            .find(|glyph| glyph.character == character)
    }
}

impl fmt::Debug for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Font")
            .field("size", &self.size)
            .field("glyph_count", &self.glyphs.len())
            .finish()
    }
}

include!(concat!(env!("OUT_DIR"), "/generated_font.rs"));
