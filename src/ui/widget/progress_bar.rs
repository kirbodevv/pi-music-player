use crate::{
    renderer::{Rect, Renderer, Size, color::Color},
    ui::widget::{Handle, Widget},
};

#[derive(Default)]
pub struct ProgressBar {
    rect: Rect,
    state: Handle<ProgressBarState>,
    last_version: u64,
    dirty: bool,
}

pub struct ProgressBarState {
    min_value: f64,
    max_value: f64,
    value: f64,
    background_color: Color,
    progress_color: Color,
}

impl ProgressBarState {
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
        self.value = self.value.clamp(self.min_value, self.max_value);
    }

    pub fn set_min_value(&mut self, min_value: f64) {
        self.min_value = min_value;
    }

    pub fn set_max_value(&mut self, max_value: f64) {
        self.max_value = max_value;
    }

    pub fn set_bounds(&mut self, min: f64, max: f64) {
        self.min_value = min;
        self.max_value = max;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_progress_color(&mut self, color: Color) {
        self.progress_color = color;
    }
}

impl Default for ProgressBarState {
    fn default() -> Self {
        Self {
            min_value: 0.,
            max_value: 100.,
            value: 0.,
            background_color: Color::rgb(0, 0, 0),
            progress_color: Color::rgb(255, 0, 0),
        }
    }
}

impl ProgressBar {
    pub fn min(self, min_value: f64) -> Self {
        self.state.modify(|state| state.min_value = min_value);
        self
    }

    pub fn max(self, max_value: f64) -> Self {
        self.state.modify(|state| state.max_value = max_value);
        self
    }

    pub fn handle(&self) -> Handle<ProgressBarState> {
        self.state.clone()
    }
}

impl Widget for ProgressBar {
    fn preferred_size(&self) -> Size {
        let bounds = self.bounds();

        Size {
            width: bounds.width,
            height: bounds.height,
        }
    }

    fn bounds(&self) -> Rect {
        self.rect
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.rect = rect;
        self.dirty = true;
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
        let bounds = self.bounds();
        let ProgressBarState {
            min_value,
            max_value,
            value,
            background_color,
            progress_color,
        } = *self.state.get();

        let progress = (value - min_value) / (max_value - min_value);
        let progress_width = bounds.width as f64 * progress;

        let progress_bounds = Rect::new(
            bounds.x,
            bounds.y,
            progress_width.round() as usize,
            bounds.height,
        );

        renderer.fill_rect(bounds, background_color);
        renderer.fill_rect(progress_bounds, progress_color);
    }
}

pub fn progress_bar() -> ProgressBar {
    ProgressBar::default()
}
