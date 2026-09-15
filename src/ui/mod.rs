pub mod screen;
pub mod widget;

use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Duration};

use crate::{
    event::InputEvent,
    music::player::AudioPlayer,
    renderer::{Renderer, color::Color},
    ui::screen::{Screen, ScreenEvent, ScreenId, launcher::Launcher},
};

pub struct Ui {
    screens: HashMap<ScreenId, Box<dyn Screen>>,
    screen: ScreenId,
    need_to_clear: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAction {
    Previous,
    PlayPause,
    Next,
}

impl Ui {
    pub fn new<A>(player: Rc<RefCell<A>>) -> Self
    where
        A: AudioPlayer + 'static,
    {
        let launcher = Launcher::new(player);

        let mut screens = HashMap::<ScreenId, Box<dyn Screen>>::new();
        screens.insert(ScreenId::Launcher, Box::new(launcher));

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

    pub fn update(&mut self, dt: Duration) {
        if let Some(screen) = self.screens.get_mut(&self.screen) {
            screen.update(dt);
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

    pub fn handle_input(&mut self, event: InputEvent) -> ScreenEvent {
        let ui_event = if let Some(screen) = self.screens.get_mut(&self.screen) {
            screen.handle_input(&event)
        } else {
            ScreenEvent::None
        };

        match &ui_event {
            ScreenEvent::None | ScreenEvent::Player(_) => {}

            ScreenEvent::Open(screen) => {
                self.change_screen(screen.clone());
            }

            ScreenEvent::Back => {
                self.change_screen(ScreenId::Launcher);
            }
        }

        ui_event
    }

    pub fn get_screen(&self, screen: ScreenId) -> &Box<dyn Screen> {
        let screen = self.screens.get(&screen).unwrap();
        screen
    }
}
