use crate::{
    event::InputEvent,
    renderer::{FONT_32, Rect, Renderer, color::Color, font::Font},
    ui::{UiEvent, widget::Widget},
};

const DEFAULT_FONT: &Font = &FONT_32;

pub struct Label {
    rect: Rect,
    text: String,
    color: Color,
    font: &'static Font,
    text_align: TextAlign,
    vertical_align: VerticalAlign,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum VerticalAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            rect: Rect::default(),
            text: String::new(),
            color: Color::WHITE,
            font: DEFAULT_FONT,
            text_align: TextAlign::default(),
            vertical_align: VerticalAlign::default(),
        }
    }
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn with_font(self, font: &'static Font) -> Self {
        Self { font, ..self }
    }

    pub fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn with_bounds(self, rect: Rect) -> Self {
        Self { rect, ..self }
    }

    pub fn with_text_align(self, text_align: TextAlign) -> Self {
        Self { text_align, ..self }
    }

    pub fn with_vertical_align(self, vertical_align: VerticalAlign) -> Self {
        Self {
            vertical_align,
            ..self
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
        let size = self.font.measure(&self.text);

        let x = match self.text_align {
            TextAlign::Left => self.rect.x,
            TextAlign::Center => self.rect.x + (self.rect.width - size.width) / 2,
            TextAlign::Right => self.rect.x + self.rect.width - size.width,
        };
        let y = match self.vertical_align {
            VerticalAlign::Top => self.rect.y + size.height,
            VerticalAlign::Center => self.rect.y + (self.rect.height + size.height) / 2,
            VerticalAlign::Bottom => self.rect.y + self.rect.height,
        };

        renderer.draw_text(x, y, &self.text, self.font, self.color);
    }
}
