use crate::renderer::color::Color;

pub mod color;
pub mod font;
pub mod primitives;
pub mod text;

pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 320;

pub use font::{FONT_12, FONT_16, FONT_20, FONT_24, FONT_32};

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    pub fn intersection(self, other: Rect) -> Rect {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);

        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);

        if x2 <= x1 || y2 <= y1 {
            return Rect::new(x1, y1, 0, 0);
        }

        Rect::new(x1, y1, x2 - x1, y2 - y1)
    }
}

pub struct Renderer {
    pixels: Vec<u16>,

    dirty: bool,
    dirty_min_x: usize,
    dirty_min_y: usize,
    dirty_max_x: usize,
    dirty_max_y: usize,

    clip_rect: Rect,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; WIDTH * HEIGHT],

            dirty: false,
            dirty_min_x: WIDTH,
            dirty_min_y: HEIGHT,
            dirty_max_x: 0,
            dirty_max_y: 0,

            clip_rect: Rect::new(0, 0, WIDTH, HEIGHT),
        }
    }

    pub fn with_clip<F>(&mut self, rect: Rect, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let previous = self.clip_rect;
        self.clip_rect = self.clip_rect.intersection(rect);
        f(self);
        self.clip_rect = previous;
    }

    pub fn pixels(&self) -> &[u16] {
        &self.pixels
    }

    pub fn clear(&mut self, color: Color) {
        let color = color.to_rgb565();

        for pixel in &mut self.pixels {
            *pixel = color;
        }

        self.mark_dirty(Rect {
            x: 0,
            y: 0,
            width: WIDTH,
            height: HEIGHT,
        });
    }

    fn blend_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        if color.a == 0 {
            return;
        }

        if color.a == 255 {
            self.pixel(x, y, color);
            return;
        }

        let index = y * WIDTH + x;

        let background = Color::from(self.pixels[index]);

        let alpha = color.a as u16;
        let inv_alpha = 255 - alpha;

        let r = (color.r as u16 * alpha + background.r as u16 * inv_alpha) / 255;

        let g = (color.g as u16 * alpha + background.g as u16 * inv_alpha) / 255;

        let b = (color.b as u16 * alpha + background.b as u16 * inv_alpha) / 255;

        self.pixel(x, y, Color::rgb(r as u8, g as u8, b as u8));
    }

    pub fn pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        let clip = self.clip_rect;

        if x < clip.x || y < clip.y || x >= clip.x + clip.width || y >= clip.y + clip.height {
            return;
        }

        let color = color.to_rgb565();
        let index = y * WIDTH + x;

        if self.pixels[index] == color {
            return;
        }

        self.pixels[index] = color;

        self.mark_dirty(Rect {
            x,
            y,
            width: 1,
            height: 1,
        });
    }

    fn mark_dirty(&mut self, rect: Rect) {
        if rect.width == 0 || rect.height == 0 {
            return;
        }

        let x2 = (rect.x + rect.width - 1).min(WIDTH - 1);
        let y2 = (rect.y + rect.height - 1).min(HEIGHT - 1);

        if !self.dirty {
            self.dirty = true;
            self.dirty_min_x = rect.x;
            self.dirty_min_y = rect.y;
            self.dirty_max_x = x2;
            self.dirty_max_y = y2;
            return;
        }

        self.dirty_min_x = self.dirty_min_x.min(rect.x);
        self.dirty_min_y = self.dirty_min_y.min(rect.y);
        self.dirty_max_x = self.dirty_max_x.max(x2);
        self.dirty_max_y = self.dirty_max_y.max(y2);
    }

    pub fn take_dirty(&mut self) -> Option<Rect> {
        if !self.dirty {
            return None;
        }

        let rect = Rect {
            x: self.dirty_min_x,
            y: self.dirty_min_y,
            width: self.dirty_max_x - self.dirty_min_x + 1,
            height: self.dirty_max_y - self.dirty_min_y + 1,
        };

        self.dirty = false;
        self.dirty_min_x = WIDTH;
        self.dirty_min_y = HEIGHT;
        self.dirty_max_x = 0;
        self.dirty_max_y = 0;

        Some(rect)
    }
}
