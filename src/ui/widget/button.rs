use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{Icon, Rect, Renderer, Size, font::Font},
    ui::{
        screen::Transition,
        widget::{ButtonStyle, Label, LayoutParams, TextAlign, VerticalAlign, Widget},
    },
};

pub type ButtonIconHandle = Rc<RefCell<Option<Icon>>>;

pub struct Button {
    rect: Rect,
    pressed: bool,

    label: Option<Label>,
    icon: ButtonIconHandle,
    last_icon: Option<Icon>,
    icon_padding: usize,

    callback: Option<Box<dyn Fn(&mut Context) -> Transition>>,

    style: ButtonStyle,
    dirty: bool,
}

impl Button {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0, 0, 0, 0),
            pressed: false,
            label: None,
            icon: Rc::new(RefCell::new(None)),
            last_icon: None,
            icon_padding: 0,
            callback: None,
            style: ButtonStyle::default(),
            dirty: true,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        let mut label = Label::new(text)
            .with_text_align(TextAlign::Center)
            .with_vertical_align(VerticalAlign::Center);

        label.set_bounds(self.rect);
        self.label = Some(label);
        self
    }

    pub fn with_text_font(mut self, font: &'static Font) -> Self {
        if let Some(label) = self.label {
            self.label = Some(label.with_font(font));
        }
        self
    }

    pub fn with_icon(mut self, icon: Icon, padding: usize) -> Self {
        *self.icon.borrow_mut() = Some(icon);
        self.icon_padding = padding;
        self
    }

    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.rect.x
            && x < self.rect.x + self.rect.width
            && y >= self.rect.y
            && y < self.rect.y + self.rect.height
    }

    pub fn on_click(mut self, callback: impl Fn(&mut Context) -> Transition + 'static) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }

    pub fn with_bounds(mut self, rect: Rect) -> Self {
        self.set_bounds(rect);
        self
    }

    fn invoke_callback(&mut self, ctx: &mut Context) -> Transition {
        if let Some(callback) = &self.callback {
            callback(ctx)
        } else {
            Transition::None
        }
    }

    fn render_rounded_button(&self, renderer: &mut Renderer) {
        let (background, light, dark) = match self.pressed {
            false => (self.style.background, self.style.light, self.style.dark),
            true => (
                self.style.pressed_background,
                self.style.pressed_light,
                self.style.pressed_dark,
            ),
        };

        renderer.draw_rounded_rect_3d(
            self.rect,
            self.style.radius,
            self.style.border_width,
            background,
            light,
            dark,
        );

        if let Some(label) = &self.label {
            label.render(renderer);
        }

        if let Some(icon) = self.icon.borrow().as_ref() {
            renderer.draw_icon(
                *icon,
                self.rect.pad(self.icon_padding),
                self.style.background.invert(),
            );
        }
    }

    fn render_square_button(&self, renderer: &mut Renderer) {
        let (background, light, dark) = match self.pressed {
            false => (self.style.background, self.style.light, self.style.dark),
            true => (
                self.style.pressed_background,
                self.style.pressed_light,
                self.style.pressed_dark,
            ),
        };

        renderer.fill_rect(self.rect, background);

        renderer.fill_rect(
            Rect::new(self.rect.x, self.rect.y, self.rect.width, 2),
            light,
        );

        renderer.fill_rect(
            Rect::new(self.rect.x, self.rect.y, 2, self.rect.height),
            light,
        );

        renderer.fill_rect(
            Rect::new(
                self.rect.x,
                self.rect.y + self.rect.height.saturating_sub(2),
                self.rect.width,
                2,
            ),
            dark,
        );

        renderer.fill_rect(
            Rect::new(
                self.rect.x + self.rect.width.saturating_sub(2),
                self.rect.y,
                2,
                self.rect.height,
            ),
            dark,
        );

        if let Some(label) = &self.label {
            label.render(renderer);
        }
    }

    pub fn icon_handle(&self) -> ButtonIconHandle {
        self.icon.clone()
    }
}

impl Widget for Button {
    fn update(&mut self, _dt: Duration) {
        let changed = {
            let icon = self.icon.borrow();
            *icon != self.last_icon
        };

        if changed {
            self.last_icon = self.icon.borrow().clone();
            self.dirty = true;
        }
    }

    fn layout_params(&self) -> LayoutParams {
        LayoutParams::default()
    }

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

        if let Some(label) = &mut self.label {
            label.set_bounds(rect);
        }

        self.dirty = true;
    }

    fn handle_input(&mut self, event: &InputEvent, ctx: &mut Context) -> Transition {
        match *event {
            InputEvent::PointDown { x, y } => {
                if self.contains(x as usize, y as usize) {
                    if !self.pressed {
                        self.pressed = true;
                        self.dirty = true;
                    }
                    return Transition::None;
                }
            }

            InputEvent::PointUp { x, y } => {
                if self.pressed {
                    self.pressed = false;
                    self.dirty = true;

                    if self.contains(x as usize, y as usize) {
                        return self.invoke_callback(ctx);
                    }

                    return Transition::None;
                }
            }

            InputEvent::PointMove { .. } => {}
        }

        Transition::None
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
        if self.style.radius > 0 {
            self.render_rounded_button(renderer);
        } else {
            self.render_square_button(renderer);
        }
    }
}
