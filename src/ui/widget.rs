use std::time::Duration;

use crate::{
    event::InputEvent,
    renderer::{Rect, Renderer, Size},
    ui::{UiEvent, layout::LayoutParams},
};

pub trait Widget {
    fn bounds(&self) -> Rect;

    fn set_bounds(&mut self, rect: Rect);

    fn preferred_size(&self) -> Size;

    fn layout_params(&self) -> LayoutParams;

    fn handle_input(&mut self, event: &InputEvent) -> UiEvent;

    fn render(&self, renderer: &mut Renderer);

    fn update(&mut self, _dt: Duration) {}
}
