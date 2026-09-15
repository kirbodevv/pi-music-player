pub mod launcher;

use crate::{event::InputEvent, renderer::Renderer, ui::PlayerAction};

pub trait Screen {
    fn update(&mut self);

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
