use crate::renderer::Renderer;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Pressed,
}

pub struct Button {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,

    pub color: u16,
    pub pressed_color: u16,

    state: ButtonState,
}

impl Button {
    pub fn new(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u16,
        pressed_color: u16,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
            pressed_color,
            state: ButtonState::Normal,
        }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.state = if pressed {
            ButtonState::Pressed
        } else {
            ButtonState::Normal
        };
    }

    pub fn render(&self, renderer: &mut Renderer) {
        let color = match self.state {
            ButtonState::Normal => self.color,
            ButtonState::Pressed => self.pressed_color,
        };

        renderer.fill_rect(self.x, self.y, self.width, self.height, color);
    }
}
