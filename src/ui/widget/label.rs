use std::time::Duration;

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

const MARQUEE_PAUSE: Duration = Duration::from_millis(1500);
const MARQUEE_SPEED: f32 = 30.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarqueePhase {
    PauseStart,
    Forward,
    PauseEnd,
    Backward,
}

impl Default for MarqueePhase {
    fn default() -> Self {
        MarqueePhase::PauseStart
    }
}

#[derive(Default)]
pub struct Label {
    rect: Rect,
    text_align: TextAlign,
    vertical_align: VerticalAlign,
    wrap: bool,

    state: Handle<LabelState>,
    last_version: u64,
    dirty: bool,

    marquee_offset: f32,
    marquee_phase: MarqueePhase,
    marquee_timer: Duration,
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

    pub fn wrap(self, wrap: bool) -> Self {
        Self { wrap, ..self }
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

    fn reset_marquee(&mut self) {
        self.marquee_offset = 0.0;
        self.marquee_phase = MarqueePhase::PauseStart;
        self.marquee_timer = Duration::ZERO;
    }

    fn marquee_max_offset(&self) -> f32 {
        if self.wrap {
            return 0.0;
        }

        let state = self.state.get();
        let lines: Vec<&str> = state.text.lines().collect();

        if lines.len() != 1 {
            return 0.0;
        }

        let line_width = state.font.measure_line(lines[0]).width as f32;
        let available = self.rect.width as f32;

        (line_width - available).max(0.0)
    }

    fn advance_marquee(&mut self, dt: Duration) -> bool {
        let max_offset = self.marquee_max_offset();

        if max_offset <= 0.0 {
            let was_active =
                self.marquee_offset != 0.0 || self.marquee_phase != MarqueePhase::PauseStart;
            self.reset_marquee();
            return was_active;
        }

        match self.marquee_phase {
            MarqueePhase::PauseStart => {
                self.marquee_timer += dt;

                if self.marquee_timer >= MARQUEE_PAUSE {
                    self.marquee_timer = Duration::ZERO;
                    self.marquee_phase = MarqueePhase::Forward;
                }

                false
            }

            MarqueePhase::Forward => {
                self.marquee_offset += MARQUEE_SPEED * dt.as_secs_f32();

                if self.marquee_offset >= max_offset {
                    self.marquee_offset = max_offset;
                    self.marquee_phase = MarqueePhase::PauseEnd;
                    self.marquee_timer = Duration::ZERO;
                }

                true
            }

            MarqueePhase::PauseEnd => {
                self.marquee_timer += dt;

                if self.marquee_timer >= MARQUEE_PAUSE {
                    self.marquee_timer = Duration::ZERO;
                    self.marquee_phase = MarqueePhase::Backward;
                }

                false
            }

            MarqueePhase::Backward => {
                self.marquee_offset -= MARQUEE_SPEED * dt.as_secs_f32();

                if self.marquee_offset <= 0.0 {
                    self.marquee_offset = 0.0;
                    self.marquee_phase = MarqueePhase::PauseStart;
                    self.marquee_timer = Duration::ZERO;
                }

                true
            }
        }
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
        if rect.width != self.rect.width {
            self.reset_marquee();
        }

        self.rect = rect;
        self.dirty = true;
    }

    fn handle_input(&mut self, _event: &InputEvent, _ctx: &mut Context) -> Transition {
        Transition::None
    }

    fn update(&mut self, dt: Duration) {
        let version = self.state.version();

        if version != self.last_version {
            self.last_version = version;
            self.dirty = true;
            self.reset_marquee();
        }

        if self.advance_marquee(dt) {
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

        let computed_lines: Vec<String> = if self.wrap {
            wrap_text(text, state.font, self.rect.width)
        } else {
            text.lines().map(str::to_string).collect()
        };

        let lines: Vec<&str> = computed_lines.iter().map(String::as_str).collect();

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

        let marquee_active = self.marquee_max_offset() > 0.0;

        let rect = self.rect;

        renderer.with_clip(rect, |renderer| {
            for (index, line) in lines.iter().enumerate() {
                let line_size = state.font.measure_line(line);

                let x: i32 = if marquee_active {
                    rect.x as i32 - self.marquee_offset.round() as i32
                } else {
                    match self.text_align {
                        TextAlign::Left => rect.x as i32,

                        TextAlign::Center => {
                            (rect.x + rect.width.saturating_sub(line_size.width) / 2) as i32
                        }

                        TextAlign::Right => {
                            (rect.x + rect.width.saturating_sub(line_size.width)) as i32
                        }
                    }
                };

                let y = start_y + index * line_height;

                const TEXT_VERTICAL_OFFSET: i32 = 2;
                let baseline = (y + line_size.height) as i32 + TEXT_VERTICAL_OFFSET;
                renderer.draw_text_signed(x, baseline as usize, line, state.font, state.color);
            }
        });
    }
}

fn wrap_text(text: &str, font: &Font, max_width: usize) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    let mut wrapped = Vec::new();

    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            wrapped.push(String::new());
            continue;
        }

        let mut current = String::new();

        for word in paragraph.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_string()
            } else {
                format!("{current} {word}")
            };

            let width = font.measure_line(&candidate).width;

            if width > max_width && !current.is_empty() {
                wrapped.push(std::mem::take(&mut current));
                current = word.to_string();
            } else {
                current = candidate;
            }
        }

        wrapped.push(current);
    }

    wrapped
}

pub fn label() -> Label {
    Label::default()
}
