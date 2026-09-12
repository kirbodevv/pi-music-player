use crate::renderer::{
    HEIGHT, Renderer, WIDTH,
    color::Color,
    font::{Font, Glyph},
};

impl Renderer {
    fn draw_glyph(&mut self, x: i32, y: i32, glyph: &Glyph, color: Color) {
        let glyph_x = x + glyph.bearing_x;
        let glyph_y = y - (glyph.bearing_y + glyph.height as i32);

        for gy in 0..glyph.height {
            for gx in 0..glyph.width {
                let index = gy * glyph.width + gx;
                let alpha = glyph.bitmap[index];

                if alpha == 0 {
                    continue;
                }

                let px = glyph_x + gx as i32;
                let py = glyph_y + gy as i32;

                if px < 0 || py < 0 {
                    continue;
                }

                let px = px as usize;
                let py = py as usize;

                if px >= WIDTH || py >= HEIGHT {
                    continue;
                }

                let alpha = glyph.bitmap[index];

                if alpha != 0 {
                    self.blend_pixel(px as usize, py as usize, color.with_alpha(alpha));
                }
            }
        }
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, font: &Font, color: Color) {
        let mut cursor_x = x;

        for ch in text.chars() {
            let Some(glyph) = font.glyph(ch) else {
                continue;
            };

            self.draw_glyph(cursor_x as i32, y as i32, glyph, color);

            cursor_x += glyph.advance.round() as usize;
        }
    }
}
