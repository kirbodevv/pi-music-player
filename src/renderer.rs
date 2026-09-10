pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 320;

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn union(self, other: Rect) -> Rect {
        if self.is_empty() {
            return other;
        }

        if other.is_empty() {
            return self;
        }

        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);

        let x2 = (self.x + self.width).max(other.x + other.width);
        let y2 = (self.y + self.height).max(other.y + other.height);

        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn expand(self, amount: usize) -> Rect {
        if self.is_empty() {
            return self;
        }

        let x = self.x.saturating_sub(amount);
        let y = self.y.saturating_sub(amount);

        let right = (self.x + self.width + amount).min(WIDTH);
        let bottom = (self.y + self.height + amount).min(HEIGHT);

        Rect {
            x,
            y,
            width: right - x,
            height: bottom - y,
        }
    }
}

pub struct Renderer {
    pub pixels: Vec<u16>,
    dirty: Rect,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; WIDTH * HEIGHT],
            dirty: Rect::empty(),
        }
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

        let index = y * WIDTH + x;

        if self.pixels[index] != color {
            self.pixels[index] = color;

            self.dirty = self.dirty.union(Rect {
                x,
                y,
                width: 1,
                height: 1,
            });
        }
    }

    pub fn clear_rect(&mut self, rect: Rect, color: u16) {
        if rect.is_empty() {
            return;
        }

        let x1 = rect.x.min(WIDTH);
        let y1 = rect.y.min(HEIGHT);
        let x2 = (rect.x + rect.width).min(WIDTH);
        let y2 = (rect.y + rect.height).min(HEIGHT);

        for y in y1..y2 {
            let start = y * WIDTH + x1;
            let end = y * WIDTH + x2;

            for pixel in &mut self.pixels[start..end] {
                *pixel = color;
            }
        }

        self.dirty = self.dirty.union(Rect {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        });
    }

    pub fn clear(&mut self, color: u16) {
        self.pixels.fill(color);

        self.dirty = Rect {
            x: 0,
            y: 0,
            width: WIDTH,
            height: HEIGHT,
        };
    }

    pub fn mark_dirty(&mut self, rect: Rect) {
        self.dirty = self.dirty.union(rect);
    }

    pub fn take_dirty(&mut self) -> Option<Rect> {
        if self.dirty.is_empty() {
            None
        } else {
            let dirty = self.dirty;
            self.dirty = Rect::empty();
            Some(dirty)
        }
    }

    pub fn rgb565(r: u8, g: u8, b: u8) -> u16 {
        let r = (r as u16 >> 3) << 11;
        let g = (g as u16 >> 2) << 5;
        let b = b as u16 >> 3;

        r | g | b
    }
}
