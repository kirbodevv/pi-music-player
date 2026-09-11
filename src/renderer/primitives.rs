use crate::renderer::color::Color;

use super::{HEIGHT, Renderer, WIDTH};

impl Renderer {
    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        let x_end = (x + width).min(WIDTH);
        let y_end = (y + height).min(HEIGHT);

        for py in y..y_end {
            for px in x..x_end {
                self.pixel(px, py, color);
            }
        }
    }

    pub fn rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        if width == 0 || height == 0 {
            return;
        }

        self.line(x, y, x + width - 1, y, color);
        self.line(x, y + height - 1, x + width - 1, y + height - 1, color);

        self.line(x, y, x, y + height - 1, color);
        self.line(x + width - 1, y, x + width - 1, y + height - 1, color);
    }

    pub fn line(&mut self, mut x0: usize, mut y0: usize, x1: usize, y1: usize, color: Color) {
        let dx = (x1 as isize - x0 as isize).abs();
        let sx = if x0 < x1 { 1 } else { -1 };

        let dy = -(y1 as isize - y0 as isize).abs();
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.pixel(x0, y0, color);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;

            if e2 >= dy {
                err += dy;
                x0 = (x0 as isize + sx) as usize;
            }

            if e2 <= dx {
                err += dx;
                y0 = (y0 as isize + sy) as usize;
            }
        }
    }
}
