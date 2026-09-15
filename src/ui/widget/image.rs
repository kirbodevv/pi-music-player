use std::time::Duration;

use crate::{
    event::InputEvent,
    renderer::{Image, Rect, Renderer, Size},
    ui::{
        screen::Transition,
        widget::{LayoutParams, Widget},
    },
};

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Clone)]
pub struct ImageHandle {
    image: Rc<RefCell<Image>>,
}

impl ImageHandle {
    pub fn get(&self) -> Ref<'_, Image> {
        self.image.borrow()
    }

    pub fn get_mut(&self) -> RefMut<'_, Image> {
        self.image.borrow_mut()
    }

    pub fn set(&self, image: Image) {
        *self.image.borrow_mut() = image;
    }
}

pub struct ImageWidget {
    image: Rc<RefCell<Image>>,
    rect: Rect,
}

impl ImageWidget {
    pub fn new(image: Image) -> Self {
        let size = Size {
            width: image.width,
            height: image.height,
        };

        Self {
            rect: Rect::new(0, 0, size.width, size.height),
            image: Rc::new(RefCell::new(image)),
        }
    }

    pub fn with_bounds(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    pub fn handle(&self) -> ImageHandle {
        ImageHandle {
            image: Rc::clone(&self.image),
        }
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
            width: self.image.borrow().width,
            height: self.image.borrow().height,
        }
    }

    fn layout_params(&self) -> LayoutParams {
        LayoutParams::default()
    }

    fn handle_input(&mut self, _event: &InputEvent) -> Transition {
        Transition::None
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.draw_image(self.rect.x, self.rect.y, &self.image.borrow());
    }

    fn update(&mut self, _dt: Duration) {}
}
