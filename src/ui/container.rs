use std::time::Duration;

use crate::{
    event::InputEvent,
    renderer::{Rect, Renderer},
    ui::{UiEvent, widget::Widget},
};

pub struct Container {
    rect: Rect,
    children: Vec<Box<dyn Widget>>,

    spacing: usize,
    padding: usize,
}

impl Container {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            children: Vec::new(),
            spacing: 0,
            padding: 0,
        }
    }

    pub fn with_child<W: Widget + 'static>(mut self, widget: W) -> Self {
        self.children.push(Box::new(widget));
        self.layout_vertical();
        self
    }

    pub fn with_spacing(mut self, spacing: usize) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    fn layout_vertical(&mut self) {
        let mut y = self.rect.y + self.padding;

        for child in &mut self.children {
            let bounds = child.bounds();

            child.set_bounds(Rect {
                x: self.rect.x + self.padding,
                y,
                width: self.rect.width.saturating_sub(self.padding * 2),
                height: bounds.height,
            });

            y += bounds.height + self.spacing;
        }
    }
}

impl Widget for Container {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.rect = rect;
        self.layout_vertical();
    }

    fn handle_input(&mut self, event: &InputEvent) -> UiEvent {
        for child in self.children.iter_mut().rev() {
            let event = child.handle_input(event);

            if event != UiEvent::None {
                return event;
            }
        }
        UiEvent::None
    }

    fn render(&self, renderer: &mut Renderer) {
        let bounds = self.bounds();

        renderer.with_clip(bounds, |renderer| {
            for child in &self.children {
                child.render(renderer);
            }
        });
    }

    fn update(&mut self, dt: Duration) {
        for child in &mut self.children {
            child.update(dt);
        }
    }
}
