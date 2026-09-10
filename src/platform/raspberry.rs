#[path = "framebuffer.rs"]
mod framebuffer;

use framebuffer::Framebuffer;

pub struct RaspberryPlatform {
    framebuffer: Framebuffer,
}

impl RaspberryPlatform {
    pub fn new() -> Self {
        Self {
            framebuffer: Framebuffer::open().expect("Failed to open /dev/fb1"),
        }
    }

    pub fn is_running(&mut self) -> bool {
        true
    }

    pub fn present(&mut self, pixels: &[u16]) {
        self.framebuffer.present(pixels);
    }
}
