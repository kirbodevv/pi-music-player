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

pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Font {
    pub fn glyph(&self, character: char) -> Option<&Glyph> {
        self.glyphs
            .iter()
            .flatten()
            .find(|glyph| glyph.character == character)
    }

    pub fn measure(&self, text: &str) -> Size {
        let mut width = 0;
        let mut height = 0;

        for c in text.chars() {
            let Some(glyph) = self.glyph(c) else {
                continue;
            };

            width += glyph.advance.ceil() as usize;
            height = height.max(glyph.height as usize);
        }

        Size { width, height }
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
