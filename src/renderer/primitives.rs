use crate::renderer::{Rect, color::Color};

use super::{HEIGHT, Renderer, WIDTH};

impl Renderer {
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let x_end = (rect.x + rect.width).min(WIDTH);
        let y_end = (rect.y + rect.height).min(HEIGHT);

        for py in rect.y..y_end {
            for px in rect.x..x_end {
                self.pixel(px, py, color);
            }
        }
    }

    pub fn rect(&mut self, rect: Rect, color: Color) {
        if rect.width == 0 || rect.height == 0 {
            return;
        }

        self.line(rect.x, rect.y, rect.x + rect.width - 1, rect.y, color);
        self.line(
            rect.x,
            rect.y + rect.height - 1,
            rect.x + rect.width - 1,
            rect.y + rect.height - 1,
            color,
        );

        self.line(rect.x, rect.y, rect.x, rect.y + rect.height - 1, color);
        self.line(
            rect.x + rect.width - 1,
            rect.y,
            rect.x + rect.width - 1,
            rect.y + rect.height - 1,
            color,
        );
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
