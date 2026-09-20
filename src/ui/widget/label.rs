use std::{cell::RefCell, rc::Rc};

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{FONT_32, Rect, Renderer, Size, color::Color, font::Font},
    ui::{
        screen::Transition,
        widget::{LayoutParams, Widget},
    },
};

const DEFAULT_FONT: &Font = &FONT_32;

pub type LabelHandle = Rc<RefCell<String>>;

pub struct Label {
    rect: Rect,
    text: LabelHandle,
    color: Color,
    font: &'static Font,
    text_align: TextAlign,
    vertical_align: VerticalAlign,
    background: Option<Color>,
    last_text: String,
    dirty: bool,
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
            text: Rc::new(RefCell::new(String::new())),
            color: Color::WHITE,
            font: DEFAULT_FONT,
            text_align: TextAlign::default(),
            vertical_align: VerticalAlign::default(),
            background: None,
            last_text: String::new(),
            dirty: true,
        }
    }
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Rc::new(RefCell::new(text.into())),
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

    /// Sets an opaque background color that gets painted behind the text
    /// every time this label redraws.
    ///
    /// This is required for any label that isn't immediately preceded by an
    /// opaque fill in the same render call (e.g. plain text sitting directly
    /// on top of a screen/container background). Without it, anti-aliased
    /// glyph edges get alpha-blended onto whatever was already there every
    /// time the label redraws, which - since the same edge gets blended
    /// again and again onto its own previous result - keeps converging
    /// towards the full glyph color until it looks blown out/oversharpened.
    pub fn with_background(self, background: Color) -> Self {
        Self {
            background: Some(background),
            ..self
        }
    }

    pub fn handle(&self) -> LabelHandle {
        Rc::clone(&self.text)
    }
}

impl Widget for Label {
    fn layout_params(&self) -> LayoutParams {
        LayoutParams::default()
    }

    fn preferred_size(&self) -> Size {
        self.font.measure(&self.text.borrow())
    }

    fn bounds(&self) -> Rect {
        self.rect
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.rect = rect;
        self.dirty = true;
    }

    fn handle_input(&mut self, _event: &InputEvent, _ctx: &mut Context) -> Transition {
        Transition::None
    }

    fn update(&mut self, _dt: std::time::Duration) {
        let changed = {
            let text = self.text.borrow();
            *text != self.last_text
        };

        if changed {
            self.last_text = self.text.borrow().clone();
            self.dirty = true;
        }
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    fn render(&self, renderer: &mut Renderer) {
        if let Some(background) = self.background {
            renderer.fill_rect(self.rect, background);
        }

        let text = self.text.borrow();

        let lines: Vec<&str> = text.lines().collect();

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

            const TEXT_VERTICAL_OFFSET: i32 = 2;
            let baseline = (y + line_size.height) as i32 + TEXT_VERTICAL_OFFSET;
            renderer.draw_text(x, baseline as usize, line, self.font, self.color);
        }
    }
}
