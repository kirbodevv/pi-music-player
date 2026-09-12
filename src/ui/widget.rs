use std::time::Duration;

use crate::{
    event::InputEvent,
    renderer::{Rect, Renderer},
    ui::UiEvent,
};

pub trait Widget {
    fn bounds(&self) -> Rect;

    fn handle_input(&mut self, event: &InputEvent) -> UiEvent;

    fn render(&self, renderer: &mut Renderer);

    fn update(&mut self, dt: Duration) {}
}
