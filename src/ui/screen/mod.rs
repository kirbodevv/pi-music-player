pub mod launcher;

use std::time::Duration;

use crate::{event::InputEvent, renderer::Renderer};

pub trait Screen {
    fn update(&mut self, dt: Duration);

    fn render(&mut self, renderer: &mut Renderer);

    fn handle_input(&mut self, event: &InputEvent) -> Transition;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenId {
    Launcher,
    Music,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transition {
    None,
    Back,
    Open(ScreenId),
}
