use crate::renderer::{Rect, Renderer};

pub trait Platform {
    fn present(&mut self, renderer: &Renderer, dirty: Rect) -> std::io::Result<()>;

    fn poll_events(&mut self) -> Vec<Event>;

    fn should_close(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Touch { x: i32, y: i32 },
    TouchDown { x: i32, y: i32 },
    TouchUp { x: i32, y: i32 },
    TouchMove { x: i32, y: i32 },

    Quit,
}

#[cfg(feature = "desktop")]
pub mod desktop;

#[cfg(feature = "raspberry")]
pub mod raspberry;

#[cfg(feature = "raspberry")]
pub mod framebuffer;

#[cfg(feature = "raspberry")]
pub mod touchscreen;
