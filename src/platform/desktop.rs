use std::io;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use crate::platform::{InputEvent, Platform};
use crate::renderer::{HEIGHT, Rect, Renderer, WIDTH};

pub struct Desktop {
    window: Window,
    buffer: Vec<u32>,
    mouse_down: bool,
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
            mouse_down: false,
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

    fn poll_events(&mut self) -> Vec<InputEvent> {
        let mut events = Vec::new();

        let Some((x, y)) = self.window.get_mouse_pos(MouseMode::Clamp) else {
            return events;
        };

        let x = x as usize;
        let y = y as usize;

        let is_down = self.window.get_mouse_down(MouseButton::Left);

        match (self.mouse_down, is_down) {
            (false, true) => {
                self.mouse_down = true;

                events.push(InputEvent::TouchDown {
                    x: x as i32,
                    y: y as i32,
                });
            }

            (true, true) => {
                events.push(InputEvent::TouchMove {
                    x: x as i32,
                    y: y as i32,
                });
            }

            (true, false) => {
                self.mouse_down = false;

                events.push(InputEvent::TouchUp {
                    x: x as i32,
                    y: y as i32,
                });
            }

            (false, false) => {}
        }

        events
    }

    fn should_close(&self) -> bool {
        !self.window.is_open() || self.window.is_key_down(Key::Escape)
    }
}
