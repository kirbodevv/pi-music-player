pub mod launcher;
pub mod music;

use std::time::Duration;

use crate::{context::Context, event::InputEvent, renderer::Renderer};

pub trait Screen {
    fn on_enter(&mut self, _ctx: &mut Context) {}

    fn on_leave(&mut self, _ctx: &mut Context) {}

    fn update(&mut self, ctx: &mut Context, dt: Duration);

    fn render(&mut self, renderer: &mut Renderer);

    fn handle_input(&mut self, event: &InputEvent, ctx: &mut Context) -> Transition;

    /// Forces a full repaint of this screen on the next render. Called when
    /// the screen becomes visible again, since its previous pixels may have
    /// been clobbered by whatever was drawn while it was hidden.
    fn mark_dirty(&mut self) {}

    /// Called once per frame after rendering to reset dirty flags.
    fn clear_dirty(&mut self) {}
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
