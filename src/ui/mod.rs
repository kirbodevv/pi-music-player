pub mod screen;
pub mod widget;

use std::{collections::HashMap, time::Duration};

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{Renderer, color::Color},
    ui::screen::{Screen, ScreenId, Transition, launcher::Launcher},
};

pub struct Ui {
    screens: HashMap<ScreenId, Box<dyn Screen>>,
    screen: ScreenId,
    need_to_clear: bool,
}

impl Ui {
    pub fn new() -> Self {
        let mut screens = HashMap::<ScreenId, Box<dyn Screen>>::new();
        screens.insert(ScreenId::Launcher, Box::new(Launcher::new()));

        Self {
            screens,
            screen: ScreenId::Launcher,
            need_to_clear: true,
        }
    }

    fn change_screen(&mut self, screen: ScreenId) {
        self.screen = screen;
        self.need_to_clear = true;
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
    }

    pub fn handle_input(&mut self, event: InputEvent, ctx: &mut Context) {
        let Some(screen) = self.screens.get_mut(&self.screen) else {
            return;
        };

        let transition = screen.handle_input(&event, ctx);

        match transition {
            Transition::Open(screen) => {
                self.change_screen(screen.clone());
            }

            Transition::Back => {
                self.change_screen(ScreenId::Launcher);
            }
            Transition::None => {}
        };
    }

    pub fn get_screen(&self, screen: ScreenId) -> &Box<dyn Screen> {
        let screen = self.screens.get(&screen).unwrap();
        screen
    }
}
