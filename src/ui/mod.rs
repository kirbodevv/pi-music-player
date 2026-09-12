pub mod button;
pub mod container;
pub mod label;
pub mod layout;
pub mod widget;

use crate::{
    event::InputEvent,
    renderer::{Rect, Renderer, color::Color},
    ui::{
        container::Container,
        label::{Label, TextAlign, VerticalAlign},
        layout::{Dimension, LayoutParams},
        widget::Widget,
    },
};

use button::Button;

pub struct Ui {
    screen: Screen,

    launcher: Container,
    music: Container,
    settings: Container,

    need_to_clear: bool,
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
        const SCREEN_RECT: Rect = Rect {
            x: 0,
            y: 0,
            width: 480,
            height: 320,
        };
        let launcher = Container::new(Rect::new(0, 0, 480, 320))
            .with_padding(67)
            .with_spacing(42)
            .with_child(
                Button::new()
                    .with_color(Color::RED, Color::MAGENTA)
                    .on_click(Box::new(|| UiEvent::Open(Screen::Music))),
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fill,
                },
            )
            .with_child(
                Button::new()
                    .with_color(Color::RED, Color::MAGENTA)
                    .on_click(Box::new(|| UiEvent::Open(Screen::Settings))),
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fill,
                },
            );

        let music = Container::new(Rect::new(0, 0, 480, 320));

        let settings = Container::new(Rect::new(0, 0, 480, 320));

        Self {
            screen: Screen::Launcher,
            launcher,
            music,
            settings,
            need_to_clear: true,
        }
    }

    fn change_screen(&mut self, screen: Screen) {
        self.screen = screen;
        self.need_to_clear = true;
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        if self.need_to_clear {
            renderer.clear(Color::rgb(15, 15, 20));
            self.need_to_clear = false;
        }

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
                self.change_screen(screen);
            }

            UiEvent::Back => {
                self.change_screen(Screen::Launcher);
            }
        }
    }
}
