use std::time::Duration;

use crate::{
    platform::InputEvent,
    renderer::{Rect, Renderer},
};

pub trait Widget {
    fn bounds(&self) -> Rect;

    fn handle_input(&mut self, event: &InputEvent) -> bool;

    fn render(&self, renderer: &mut Renderer);

    fn update(&mut self, dt: Duration) {}
}
