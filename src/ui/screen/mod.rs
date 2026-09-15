pub mod launcher;

use std::time::Duration;

use crate::{event::InputEvent, renderer::Renderer, ui::PlayerAction};

pub trait Screen {
    fn update(&mut self, dt: Duration);

    fn render(&mut self, renderer: &mut Renderer);

    fn handle_input(&mut self, event: &InputEvent) -> ScreenEvent;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenId {
    Launcher,
    Music,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenEvent {
    None,
    Back,
    Open(ScreenId),
    Player(PlayerAction),
}
