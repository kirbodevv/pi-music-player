use std::{cell::RefCell, rc::Rc};

use crate::{
    renderer::{Rect, Renderer, Size, color::Color},
    ui::widget::Widget,
};

pub type ProgressBarHandle = Rc<RefCell<ProgressBarState>>;

pub struct ProgressBar {
    rect: Rect,
    state: ProgressBarHandle,
    background_color: Color,
    progress_color: Color,
}

pub struct ProgressBarState {
    min_value: f64,
    max_value: f64,
    value: f64,
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
}

impl Default for ProgressBarState {
    fn default() -> Self {
        Self {
            min_value: 0.,
            max_value: 100.,
            value: 0.,
        }
    }
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0, 0, 0, 0),
            state: Rc::new(RefCell::new(ProgressBarState::default())),
            background_color: Color::rgb(0, 0, 0),
            progress_color: Color::rgb(255, 0, 0),
        }
    }

    pub fn with_min_value(self, min_value: f64) -> Self {
        self.state.borrow_mut().min_value = min_value;
        self
    }

    pub fn with_max_value(self, max_value: f64) -> Self {
        self.state.borrow_mut().max_value = max_value;
        self
    }

    pub fn handle(&self) -> ProgressBarHandle {
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
    }

    fn render(&self, renderer: &mut Renderer) {
        let bounds = self.bounds();
        let ProgressBarState {
            min_value,
            max_value,
            value,
        } = *self.state.borrow();

        let progress = (value - min_value) / (max_value - min_value);
        let progress_width = bounds.width as f64 * progress;

        let progress_bounds = Rect::new(
            bounds.x,
            bounds.y,
            progress_width.round() as usize,
            bounds.height,
        );

        renderer.fill_rect(bounds, self.background_color);
        renderer.fill_rect(progress_bounds, self.progress_color);
    }
}
