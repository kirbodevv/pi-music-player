use std::io;

use crate::{
    event::InputEvent,
    platform::Platform,
    platform::touchscreen::Touchscreen,
    renderer::{Rect, Renderer},
};

use super::framebuffer::Framebuffer;

pub struct Raspberry {
    framebuffer: Framebuffer,
    touchscreen: Touchscreen,
}

impl Raspberry {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            framebuffer: Framebuffer::open()?,
            touchscreen: Touchscreen::open("/dev/input/event0")?,
        })
    }
}

impl Platform for Raspberry {
    fn present(&mut self, renderer: &Renderer, dirty: Rect) -> io::Result<()> {
        self.framebuffer.present(&renderer.pixels(), dirty)
    }

    fn poll_events(&mut self) -> Vec<InputEvent> {
        let mut events = Vec::new();
        for event in self.touchscreen.poll().unwrap() {
            events.push(event);
        }
        events
    }

    fn should_close(&self) -> bool {
        false
    }
}
