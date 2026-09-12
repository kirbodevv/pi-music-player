use std::time::Duration;

use crate::{
    platform::InputEvent,
    renderer::{Rect, Renderer},
    ui::{UiEvent, widget::Widget},
};

pub struct Container {
    rect: Rect,
    children: Vec<Box<dyn Widget>>,
}

impl Container {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            children: Vec::new(),
        }
    }

    pub fn with_child<W: Widget + 'static>(mut self, widget: W) -> Self {
        self.children.push(Box::new(widget));
        self
    }
}

impl Widget for Container {
    fn bounds(&self) -> Rect {
        self.rect
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
        for child in &self.children {
            child.render(renderer);
        }
    }

    fn update(&mut self, dt: Duration) {
        for child in &mut self.children {
            child.update(dt);
        }
    }
}
