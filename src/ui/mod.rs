mod button;
mod container;
mod label;
mod widget;

use crate::{
    platform::InputEvent,
    renderer::{Rect, Renderer, color::Color},
    ui::{container::Container, widget::Widget},
};

use button::Button;

pub struct Ui {
    screen: Screen,

    launcher: Container,
    music: Container,
    settings: Container,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    None,
    Open(Screen),
    Back,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Launcher,
    Music,
    Settings,
}

impl Ui {
    pub fn new() -> Self {
        let launcher = Container::new(Rect::new(0, 0, 480, 320)).with_children(vec![
            Button::new(Rect::new(40, 60, 180, 80))
                .with_color(Color::RED, Color::MAGENTA)
                .on_click(Box::new(|| UiEvent::Open(Screen::Music))),
            Button::new(Rect::new(260, 60, 180, 80)).with_color(Color::BLACK, Color::GRAY),
            Button::new(Rect::new(40, 180, 180, 80)).with_color(Color::BLUE, Color::CYAN),
            Button::new(Rect::new(260, 180, 180, 80))
                .with_color(Color::YELLOW, Color::WHITE)
                .on_click(Box::new(|| UiEvent::Open(Screen::Settings))),
        ]);

        let music = Container::new(Rect::new(0, 0, 480, 320)).with_children(vec![
            Button::new(Rect::new(0, 0, 100, 100))
                .with_color(Color::RED, Color::MAGENTA)
                .on_click(Box::new(|| UiEvent::Open(Screen::Launcher))),
        ]);

        let settings = Container::new(Rect::new(0, 0, 480, 320)).with_children(vec![
            Button::new(Rect::new(0, 0, 100, 100))
                .with_color(Color::RED, Color::MAGENTA)
                .on_click(Box::new(|| UiEvent::Open(Screen::Launcher))),
        ]);

        Self {
            screen: Screen::Launcher,
            launcher,
            music,
            settings,
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.clear(Color::rgb(15, 15, 20));

        match self.screen {
            Screen::Launcher => self.launcher.render(renderer),
            Screen::Music => self.music.render(renderer),
            Screen::Settings => self.settings.render(renderer),
        }
    }

    pub fn handle_input(&mut self, event: InputEvent) {
        let ui_event = match self.screen {
            Screen::Launcher => self.launcher.handle_input(&event),
            Screen::Music => self.music.handle_input(&event),
            Screen::Settings => self.settings.handle_input(&event),
        };

        match ui_event {
            UiEvent::None => {}

            UiEvent::Open(screen) => {
                self.screen = screen;
            }

            UiEvent::Back => {
                self.screen = Screen::Launcher;
            }
        }
    }
}
