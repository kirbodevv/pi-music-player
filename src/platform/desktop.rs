use minifb::{Window, WindowOptions};

const WIDTH: usize = 480;
const HEIGHT: usize = 320;

pub struct DesktopPlatform {
    window: Window,
    buffer: Vec<u32>,
}

impl DesktopPlatform {
    pub fn new() -> Self {
        let window = Window::new("Music Player", WIDTH, HEIGHT, WindowOptions::default())
            .expect("Failed to create window");

        Self {
            window,
            buffer: vec![0; WIDTH * HEIGHT],
        }
    }

    pub fn is_running(&mut self) -> bool {
        self.window.is_open()
    }

    pub fn present(&mut self, pixels: &[u16]) {
        for (i, &pixel) in pixels.iter().enumerate() {
            self.buffer[i] = rgb565_to_rgb888(pixel);
        }

        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .expect("Failed to update window");
    }
}

fn rgb565_to_rgb888(pixel: u16) -> u32 {
    let r = ((pixel >> 11) & 0x1f) as u32;
    let g = ((pixel >> 5) & 0x3f) as u32;
    let b = (pixel & 0x1f) as u32;

    let r = (r << 3) | (r >> 2);
    let g = (g << 2) | (g >> 4);
    let b = (b << 3) | (b >> 2);

    (r << 16) | (g << 8) | b
}
