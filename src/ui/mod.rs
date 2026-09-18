pub mod registry;
pub mod screen;
pub mod widget;

use std::{collections::HashMap, time::Duration};

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{Rect, Renderer, color::Color, icon},
    ui::screen::{Screen, ScreenId, Transition},
};

pub struct Ui {
    screens: HashMap<ScreenId, Box<dyn Screen>>,
    screen: ScreenId,
    need_to_clear: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            screens: registry::build_screens(),
            screen: ScreenId::Launcher,
            need_to_clear: true,
        }
    }

    fn change_screen(&mut self, screen: ScreenId, ctx: &mut Context) {
        if let Some(current) = self.screens.get_mut(&self.screen) {
            current.on_leave(ctx);
        }

        self.screen = screen;
        self.need_to_clear = true;

        if let Some(next) = self.screens.get_mut(&self.screen) {
            next.on_enter(ctx);
        }
    }

    pub fn update(&mut self, ctx: &mut Context, dt: Duration) {
        if let Some(screen) = self.screens.get_mut(&self.screen) {
            screen.update(ctx, dt);
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        if self.need_to_clear {
            renderer.clear(Color::rgb(15, 15, 20));
            self.need_to_clear = false;
        }

        if let Some(screen) = self.screens.get_mut(&self.screen) {
            screen.render(renderer);
        }
        renderer.draw_icon(
            icon::Icon::ArrowLeft,
            Rect::new(24 * 0, 0, 24, 24),
            Color::BLUE,
        );
        renderer.draw_icon(icon::Icon::Music, Rect::new(24 * 1, 0, 24, 24), Color::BLUE);
        renderer.draw_icon(icon::Icon::Pause, Rect::new(24 * 2, 0, 24, 24), Color::BLUE);
        renderer.draw_icon(icon::Icon::Play, Rect::new(24 * 3, 0, 24, 24), Color::BLUE);
        renderer.draw_icon(
            icon::Icon::Settings,
            Rect::new(24 * 4, 0, 24, 24),
            Color::BLUE,
        );
        renderer.draw_icon(
            icon::Icon::SkipBack,
            Rect::new(24 * 5, 0, 24, 24),
            Color::BLUE,
        );
        renderer.draw_icon(
            icon::Icon::SkipForward,
            Rect::new(24 * 6, 0, 24, 24),
            Color::BLUE,
        );
    }

    pub fn handle_input(&mut self, event: InputEvent, ctx: &mut Context) {
        let Some(screen) = self.screens.get_mut(&self.screen) else {
            return;
        };

        let transition = screen.handle_input(&event, ctx);

        match transition {
            Transition::Open(screen) => {
                self.change_screen(screen, ctx);
            }

            Transition::Back => {
                self.change_screen(ScreenId::Launcher, ctx);
            }
            Transition::None => {}
        };
    }

    pub fn get_screen(&self, screen: ScreenId) -> &Box<dyn Screen> {
        let screen = self.screens.get(&screen).unwrap();
        screen
    }
}
