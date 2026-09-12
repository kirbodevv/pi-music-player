use std::time::Duration;

use crate::{
    event::InputEvent,
    renderer::{Image, Rect, Renderer, Size},
    ui::{UiEvent, layout::LayoutParams, widget::Widget},
};

pub struct ImageWidget {
    rect: Rect,
    image: Image,
}

impl ImageWidget {
    pub fn new(image: Image) -> Self {
        let size = Size {
            width: image.width,
            height: image.height,
        };

        Self {
            rect: Rect::new(0, 0, size.width, size.height),
            image,
        }
    }

    pub fn with_bounds(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }
}

impl Widget for ImageWidget {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn preferred_size(&self) -> Size {
        Size {
            width: self.image.width,
            height: self.image.height,
        }
    }

    fn layout_params(&self) -> LayoutParams {
        LayoutParams::default()
    }

    fn handle_input(&mut self, _event: &InputEvent) -> UiEvent {
        UiEvent::None
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.draw_image(self.rect.x, self.rect.y, &self.image);
    }

    fn update(&mut self, _dt: Duration) {}
}
