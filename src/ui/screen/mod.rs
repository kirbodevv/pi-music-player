pub mod launcher;

use std::time::Duration;

use crate::{context::Context, event::InputEvent, renderer::Renderer};

pub trait Screen {
    fn update(&mut self, ctx: &mut Context, dt: Duration);

    fn render(&mut self, renderer: &mut Renderer);

    fn handle_input(&mut self, event: &InputEvent, ctx: &mut Context) -> Transition;
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
