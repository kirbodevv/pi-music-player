use std::time::Duration;

use crate::{
    platform::InputEvent,
    renderer::{Rect, Renderer},
    ui::widget::Widget,
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

    pub fn add<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget));
    }

    pub fn with_children<W: Widget + 'static>(mut self, children: Vec<W>) -> Self {
        for child in children {
            self.add(child);
        }
        self
    }
}

impl Widget for Container {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn handle_input(&mut self, event: &InputEvent) -> bool {
        for child in self.children.iter_mut().rev() {
            if child.handle_input(event) {
                return true;
            }
        }

        false
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
