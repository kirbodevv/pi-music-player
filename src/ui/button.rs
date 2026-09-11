use crate::{
    platform::InputEvent,
    renderer::{Rect, Renderer, color::Color},
    ui::widget::Widget,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonState {
    Normal,
    Pressed,
}

pub struct Button {
    rect: Rect,
    state: ButtonState,

    normal_color: Color,
    pressed_color: Color,
}

impl Button {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            state: ButtonState::Normal,

            normal_color: Color::rgb(60, 60, 70),
            pressed_color: Color::rgb(100, 100, 120),
        }
    }

    pub fn with_color(mut self, normal_color: Color, pressed_color: Color) -> Self {
        self.normal_color = normal_color;
        self.pressed_color = pressed_color;
        self
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.rect.x
            && x < self.rect.x + self.rect.width
            && y >= self.rect.y
            && y < self.rect.y + self.rect.height
    }
}

impl Widget for Button {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn handle_input(&mut self, event: &InputEvent) -> bool {
        match *event {
            InputEvent::TouchDown { x, y } => {
                if self.contains(x as usize, y as usize) {
                    self.state = ButtonState::Pressed;
                    return true;
                }
            }

            InputEvent::TouchUp { x, y } => {
                if self.state == ButtonState::Pressed {
                    self.state = ButtonState::Normal;

                    if self.contains(x as usize, y as usize) {
                        println!("Button clicked!");
                    }

                    return true;
                }
            }

            InputEvent::TouchMove { .. } => {}

            _ => {}
        }

        false
    }

    fn render(&self, renderer: &mut Renderer) {
        let color = match self.state {
            ButtonState::Normal => self.normal_color,
            ButtonState::Pressed => self.pressed_color,
        };

        renderer.fill_rect(self.rect, color);
    }
}
