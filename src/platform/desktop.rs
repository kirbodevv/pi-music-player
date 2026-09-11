use std::io;

use minifb::{Key, Window, WindowOptions};

use crate::platform::Platform;
use crate::renderer::{HEIGHT, Rect, Renderer, WIDTH};

pub struct Desktop {
    window: Window,
    buffer: Vec<u32>,
}

impl Desktop {
    pub fn new() -> io::Result<Self> {
        let window = Window::new(
            "Music Device",
            WIDTH,
            HEIGHT,
            WindowOptions {
                resize: false,
                scale: minifb::Scale::X2,
                ..WindowOptions::default()
            },
        )
        .map_err(io::Error::other)?;

        Ok(Self {
            window,
            buffer: vec![0; WIDTH * HEIGHT],
        })
    }
}

impl Platform for Desktop {
    fn present(&mut self, renderer: &Renderer, dirty: Rect) -> io::Result<()> {
        let x1 = dirty.x.min(WIDTH);
        let y1 = dirty.y.min(HEIGHT);
        let x2 = (dirty.x + dirty.width).min(WIDTH);
        let y2 = (dirty.y + dirty.height).min(HEIGHT);

        for y in y1..y2 {
            for x in x1..x2 {
                let rgb565 = renderer.pixels()[y * WIDTH + x];

                let r = ((rgb565 >> 11) & 0x1f) as u32;
                let g = ((rgb565 >> 5) & 0x3f) as u32;
                let b = (rgb565 & 0x1f) as u32;

                let r = (r << 3) | (r >> 2);
                let g = (g << 2) | (g >> 4);
                let b = (b << 3) | (b >> 2);

                self.buffer[y * WIDTH + x] = (r << 16) | (g << 8) | b;
            }
        }

        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .map_err(io::Error::other)
    }

    fn poll_events(&mut self) {}

    fn should_close(&self) -> bool {
        !self.window.is_open() || self.window.is_key_down(Key::Escape)
    }
}
