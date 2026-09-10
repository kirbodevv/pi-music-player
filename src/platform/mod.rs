use crate::renderer::{Rect, Renderer};

pub trait Platform {
    fn present(&mut self, renderer: &Renderer, dirty: Rect) -> std::io::Result<()>;

    fn poll_events(&mut self);

    fn should_close(&self) -> bool;
}

#[cfg(feature = "desktop")]
pub mod desktop;

#[cfg(feature = "raspberry")]
pub mod raspberry;

#[cfg(feature = "raspberry")]
pub mod framebuffer;

#[cfg(feature = "raspberry")]
pub mod touchscreen;
