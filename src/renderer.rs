pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 320;

pub struct Renderer {
    pub pixels: Vec<u16>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self, color: u16) {
        self.pixels.fill(color);
    }

    pub fn pixel(&mut self, x: i32, y: i32, color: u16) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        self.pixels[y * WIDTH + x] = color;
    }

    pub fn rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u16) {
        for py in y..y + height {
            for px in x..x + width {
                self.pixel(px, py, color);
            }
        }
    }

    pub fn rect_outline(&mut self, x: i32, y: i32, width: i32, height: i32, color: u16) {
        for px in x..x + width {
            self.pixel(px, y, color);
            self.pixel(px, y + height - 1, color);
        }

        for py in y..y + height {
            self.pixel(x, py, color);
            self.pixel(x + width - 1, py, color);
        }
    }

    pub fn line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: u16) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };

        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut error = dx + dy;

        loop {
            self.pixel(x0, y0, color);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * error;

            if e2 >= dy {
                error += dy;
                x0 += sx;
            }

            if e2 <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }
}

pub fn rgb565(r: u8, g: u8, b: u8) -> u16 {
    ((r as u16 >> 3) << 11) | ((g as u16 >> 2) << 5) | (b as u16 >> 3)
}
