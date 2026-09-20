use crate::{
    context::Context,
    event::InputEvent,
    renderer::{FONT_32, Rect, Renderer, Size, color::Color, font::Font},
    ui::{
        screen::Transition,
        widget::{Handle, LayoutParams, Widget},
    },
};

const DEFAULT_FONT: &Font = &FONT_32;

#[derive(Default)]
pub struct Label {
    rect: Rect,
    text_align: TextAlign,
    vertical_align: VerticalAlign,

    state: Handle<LabelState>,
    last_version: u64,
    dirty: bool,
}

pub struct LabelState {
    pub text: String,
    pub background: Option<Color>,
    pub color: Color,
    pub font: &'static Font,
}

impl Default for LabelState {
    fn default() -> Self {
        Self {
            text: String::new(),
            background: None,
            color: Color::WHITE,
            font: DEFAULT_FONT,
        }
    }
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

impl Label {
    pub fn text(self, text: impl Into<String>) -> Self {
        self.state.modify(|s| {
            s.text = text.into();
        });

        self
    }

    pub fn font(self, font: &'static Font) -> Self {
        self.state.modify(|s| {
            s.font = font;
        });

        self
    }

    pub fn color(self, color: Color) -> Self {
        self.state.modify(|state| {
            state.color = color;
        });

        self
    }

    pub fn bounds(self, rect: Rect) -> Self {
        Self { rect, ..self }
    }

    pub fn text_align(self, text_align: TextAlign) -> Self {
        Self { text_align, ..self }
    }

    pub fn vertical_align(self, vertical_align: VerticalAlign) -> Self {
        Self {
            vertical_align,
            ..self
        }
    }

    pub fn background(self, background: Color) -> Self {
        self.state.modify(|state| {
            state.background = Some(background);
        });

        self
    }

    pub fn handle(&self) -> Handle<LabelState> {
        self.state.clone()
    }
}

impl Widget for Label {
    fn layout_params(&self) -> LayoutParams {
        LayoutParams::default()
    }

    fn preferred_size(&self) -> Size {
        let state = self.state.get();
        state.font.measure(&state.text)
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
        let version = self.state.version();

        if version != self.last_version {
            self.last_version = version;
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
        let state = self.state.get();
        if let Some(background) = state.background {
            renderer.fill_rect(self.rect, background);
        }

        let text = state.text.as_str();

        let lines: Vec<&str> = text.lines().collect();

        if lines.is_empty() {
            return;
        }

        let line_height = state.font.size as usize;

        let total_height = line_height * lines.len();

        let start_y = match self.vertical_align {
            VerticalAlign::Top => self.rect.y,
            VerticalAlign::Center => {
                self.rect.y + self.rect.height.saturating_sub(total_height) / 2
            }
            VerticalAlign::Bottom => self.rect.y + self.rect.height.saturating_sub(total_height),
        };

        for (index, line) in lines.iter().enumerate() {
            let line_size = state.font.measure_line(line);

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
            renderer.draw_text(x, baseline as usize, line, state.font, state.color);
        }
    }
}

pub fn label() -> Label {
    Label::default()
}
