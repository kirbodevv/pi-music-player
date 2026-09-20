pub mod button;
pub mod container;
pub mod image;
pub mod label;
pub mod layout;
pub mod progress_bar;
pub mod style;

pub use button::*;
pub use container::*;
pub use image::*;
pub use label::*;
pub use layout::*;
pub use progress_bar::*;
pub use style::*;

use std::time::Duration;

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{Rect, Renderer, Size},
    ui::screen::Transition,
};

pub trait Widget {
    fn bounds(&self) -> Rect;

    fn set_bounds(&mut self, rect: Rect);

    fn preferred_size(&self) -> Size;

    fn layout_params(&self) -> LayoutParams {
        LayoutParams::default()
    }

    fn handle_input(&mut self, _event: &InputEvent, _ctx: &mut Context) -> Transition {
        Transition::None
    }

    fn render(&self, renderer: &mut Renderer);

    fn update(&mut self, _dt: Duration) {}

    /// Whether this widget (or any of its children) needs to be repainted.
    ///
    /// Widgets that don't override this default to "always dirty", which is
    /// the safe (if wasteful) legacy behaviour: they get redrawn every frame.
    fn is_dirty(&self) -> bool {
        true
    }

    /// Called once per frame after rendering to reset dirty flags.
    fn clear_dirty(&mut self) {}

    /// Forces this widget (and its children) to be considered dirty, so the
    /// next render fully repaints it. Used e.g. when switching screens.
    fn mark_dirty(&mut self) {}
}
