use crate::{
    event::InputEvent,
    renderer::{FONT_32, Rect, Renderer, color::Color, font::Font},
    ui::{UiEvent, layout::LayoutParams, widget::Widget},
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
    fn layout_params(&self) -> LayoutParams {
        LayoutParams::default()
    }

    fn bounds(&self) -> Rect {
        self.rect
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn handle_input(&mut self, _event: &InputEvent) -> UiEvent {
        UiEvent::None
    }

    fn render(&self, renderer: &mut Renderer) {
        let lines: Vec<&str> = self.text.lines().collect();

        if lines.is_empty() {
            return;
        }

        let line_height = self.font.size as usize;

        let total_height = line_height * lines.len();

        let start_y = match self.vertical_align {
            VerticalAlign::Top => self.rect.y,
            VerticalAlign::Center => {
                self.rect.y + self.rect.height.saturating_sub(total_height) / 2
            }
            VerticalAlign::Bottom => self.rect.y + self.rect.height.saturating_sub(total_height),
        };

        for (index, line) in lines.iter().enumerate() {
            let line_size = self.font.measure_line(line);

            let x = match self.text_align {
                TextAlign::Left => self.rect.x,

                TextAlign::Center => {
                    self.rect.x + self.rect.width.saturating_sub(line_size.width) / 2
                }

                TextAlign::Right => self.rect.x + self.rect.width.saturating_sub(line_size.width),
            };

            let y = start_y + index * line_height;

            renderer.draw_text(x, y + line_size.height, line, self.font, self.color);
        }
    }
}
