use std::io;

use crate::platform::Platform;
use crate::renderer::{Rect, Renderer};

use super::framebuffer::Framebuffer;

pub struct Raspberry {
    framebuffer: Framebuffer,
}

impl Raspberry {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            framebuffer: Framebuffer::open()?,
        })
    }
}

impl Platform for Raspberry {
    fn present(&mut self, renderer: &Renderer, dirty: Rect) -> io::Result<()> {
        self.framebuffer.present(&renderer.pixels(), dirty)
    }

    fn poll_events(&mut self) {}

    fn should_close(&self) -> bool {
        false
    }
}
