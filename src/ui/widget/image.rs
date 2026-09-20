use std::time::Duration;

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{Image, Rect, Renderer, Size},
    ui::{
        screen::Transition,
        widget::{Handle, LayoutParams, Widget},
    },
};

pub struct ImageWidget {
    rect: Rect,
    state: Handle<ImageState>,
    last_version: u64,
    dirty: bool,
}

pub struct ImageState {
    pub image: Image,
}

impl ImageWidget {
    pub fn new(image: Image) -> Self {
        let size = Size {
            width: image.width,
            height: image.height,
        };

        Self {
            rect: Rect::new(0, 0, size.width, size.height),
            state: Handle::new(ImageState { image }),
            last_version: 0,
            dirty: true,
        }
    }

    pub fn with_bounds(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    pub fn handle(&self) -> Handle<ImageState> {
        self.state.clone()
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
        let (width, height) = self.state.get().image.dimensions();
        Size { width, height }
    }

    fn layout_params(&self) -> LayoutParams {
        LayoutParams::default()
    }

    fn handle_input(&mut self, _event: &InputEvent, _ctx: &mut Context) -> Transition {
        Transition::None
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.draw_image(self.rect.x, self.rect.y, &self.state.get().image);
    }

    fn update(&mut self, _dt: Duration) {
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
}
