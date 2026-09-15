pub mod button;
pub mod container;
pub mod image;
pub mod label;
pub mod layout;
pub mod style;

pub use button::*;
pub use container::*;
pub use image::*;
pub use label::*;
pub use layout::*;
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

    fn layout_params(&self) -> LayoutParams;

    fn handle_input(&mut self, event: &InputEvent, ctx: &mut Context) -> Transition;

    fn render(&self, renderer: &mut Renderer);

    fn update(&mut self, _dt: Duration) {}
}
