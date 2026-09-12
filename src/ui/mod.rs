mod button;
mod container;
mod label;
mod widget;

use crate::{
    event::InputEvent,
    renderer::{Rect, Renderer, color::Color},
    ui::{
        container::Container,
        label::{Label, TextAlign, VerticalAlign},
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
            .with_child(
                Button::new(Rect::new(40, 60, 180, 80))
                    .with_color(Color::RED, Color::MAGENTA)
                    .on_click(Box::new(|| UiEvent::Open(Screen::Music))),
            )
            .with_child(
                Button::new(Rect::new(260, 60, 180, 80)).with_color(Color::BLACK, Color::GRAY),
            )
            .with_child(
                Button::new(Rect::new(40, 180, 180, 80)).with_color(Color::BLUE, Color::CYAN),
            )
            .with_child(
                Button::new(Rect::new(260, 180, 180, 80))
                    .with_color(Color::YELLOW, Color::WHITE)
                    .on_click(Box::new(|| UiEvent::Open(Screen::Settings))),
            )
            .with_child(
                Label::new("TL")
                    .with_bounds(SCREEN_RECT)
                    .with_text_align(TextAlign::Left)
                    .with_vertical_align(VerticalAlign::Top),
            )
            .with_child(
                Label::new("TC")
                    .with_bounds(SCREEN_RECT)
                    .with_text_align(TextAlign::Center)
                    .with_vertical_align(VerticalAlign::Top),
            )
            .with_child(
                Label::new("TR")
                    .with_bounds(SCREEN_RECT)
                    .with_text_align(TextAlign::Right)
                    .with_vertical_align(VerticalAlign::Top),
            )
            .with_child(
                Label::new("CL")
                    .with_bounds(SCREEN_RECT)
                    .with_text_align(TextAlign::Left)
                    .with_vertical_align(VerticalAlign::Center),
            )
            .with_child(
                Label::new("CC")
                    .with_bounds(SCREEN_RECT)
                    .with_text_align(TextAlign::Center)
                    .with_vertical_align(VerticalAlign::Center),
            )
            .with_child(
                Label::new("CR")
                    .with_bounds(SCREEN_RECT)
                    .with_text_align(TextAlign::Right)
                    .with_vertical_align(VerticalAlign::Center),
            )
            .with_child(
                Label::new("BL")
                    .with_bounds(SCREEN_RECT)
                    .with_text_align(TextAlign::Left)
                    .with_vertical_align(VerticalAlign::Bottom),
            )
            .with_child(
                Label::new("BC")
                    .with_bounds(SCREEN_RECT)
                    .with_text_align(TextAlign::Center)
                    .with_vertical_align(VerticalAlign::Bottom),
            )
            .with_child(
                Label::new("BR")
                    .with_bounds(SCREEN_RECT)
                    .with_text_align(TextAlign::Right)
                    .with_vertical_align(VerticalAlign::Bottom),
            );

        let music = Container::new(Rect::new(0, 0, 480, 320)).with_child(
            Button::new(Rect::new(0, 0, 100, 100))
                .with_color(Color::RED, Color::MAGENTA)
                .on_click(Box::new(|| UiEvent::Open(Screen::Launcher))),
        );

        let settings = Container::new(Rect::new(0, 0, 480, 320)).with_child(
            Button::new(Rect::new(0, 0, 100, 100))
                .with_color(Color::RED, Color::MAGENTA)
                .on_click(Box::new(|| UiEvent::Open(Screen::Launcher))),
        );

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
