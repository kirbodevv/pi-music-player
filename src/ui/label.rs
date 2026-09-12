use crate::{
    event::InputEvent,
    renderer::{FONT_32, Rect, Renderer, color::Color},
    ui::{UiEvent, widget::Widget},
};

pub struct Label {
    rect: Rect,
    text: String,
    color: Color,
}

impl Label {
    pub fn new(rect: Rect, text: impl Into<String>, color: Color) -> Self {
        Self {
            rect,
            text: text.into(),
            color,
        }
    }
}

impl Widget for Label {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn handle_input(&mut self, _event: &InputEvent) -> UiEvent {
        UiEvent::None
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.draw_text(self.rect.x, self.rect.y, &self.text, &FONT_32, self.color);
    }
}
