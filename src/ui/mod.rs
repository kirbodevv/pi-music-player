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
    root: Container,
}

enum Screen {
    Launcher,
    Music,
    Settings,
}

impl Ui {
    pub fn new() -> Self {
        let root = Container::new(Rect::new(0, 0, 480, 320)).with_children(vec![
            Button::new(Rect::new(40, 60, 180, 80)).with_color(Color::RED, Color::MAGENTA),
            Button::new(Rect::new(260, 60, 180, 80)).with_color(Color::BLACK, Color::GRAY),
            Button::new(Rect::new(40, 180, 180, 80)).with_color(Color::BLUE, Color::CYAN),
            Button::new(Rect::new(260, 180, 180, 80)).with_color(Color::YELLOW, Color::WHITE),
        ]);

        Self {
            screen: Screen::Launcher,
            root,
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.clear(Color::rgb(15, 15, 20));

        self.root.render(renderer);
    }

    pub fn handle_input(&mut self, event: InputEvent) {
        self.root.handle_input(&event);
    }
}
