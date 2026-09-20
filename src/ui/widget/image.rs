use std::time::Duration;

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{Image, Rect, Renderer, Size},
    ui::{
        screen::Transition,
        widget::{LayoutParams, Widget},
    },
};

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

struct ImageSlot {
    image: Image,
    generation: u64,
}

#[derive(Clone)]
pub struct ImageHandle {
    slot: Rc<RefCell<ImageSlot>>,
}

impl ImageHandle {
    pub fn get(&self) -> Ref<'_, Image> {
        Ref::map(self.slot.borrow(), |slot| &slot.image)
    }

    pub fn get_mut(&self) -> RefMut<'_, Image> {
        let mut slot = self.slot.borrow_mut();
        slot.generation += 1;
        RefMut::map(slot, |slot| &mut slot.image)
    }

    pub fn set(&self, image: Image) {
        let mut slot = self.slot.borrow_mut();
        slot.image = image;
        slot.generation += 1;
    }
}

pub struct ImageWidget {
    slot: Rc<RefCell<ImageSlot>>,
    rect: Rect,
    last_generation: u64,
    dirty: bool,
}

impl ImageWidget {
    pub fn new(image: Image) -> Self {
        let size = Size {
            width: image.width,
            height: image.height,
        };

        Self {
            rect: Rect::new(0, 0, size.width, size.height),
            slot: Rc::new(RefCell::new(ImageSlot {
                image,
                generation: 0,
            })),
            last_generation: 0,
            dirty: true,
        }
    }

    pub fn with_bounds(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    pub fn handle(&self) -> ImageHandle {
        ImageHandle {
            slot: Rc::clone(&self.slot),
        }
    }
}

impl Widget for ImageWidget {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.rect = rect;
        self.dirty = true;
    }

    fn preferred_size(&self) -> Size {
        let slot = self.slot.borrow();

        Size {
            width: slot.image.width,
            height: slot.image.height,
        }
    }

    fn layout_params(&self) -> LayoutParams {
        LayoutParams::default()
    }

    fn handle_input(&mut self, _event: &InputEvent, _ctx: &mut Context) -> Transition {
        Transition::None
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.draw_image(self.rect.x, self.rect.y, &self.slot.borrow().image);
    }

    fn update(&mut self, _dt: Duration) {
        let generation = self.slot.borrow().generation;

        if generation != self.last_generation {
            self.last_generation = generation;
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
}
