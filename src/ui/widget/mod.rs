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
}
