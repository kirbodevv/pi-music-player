use std::time::Duration;

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{Icon, Rect, Renderer, Size, font::Font},
    ui::{
        screen::Transition,
        widget::{ButtonStyle, Handle, Label, LayoutParams, TextAlign, VerticalAlign, Widget},
    },
};

pub struct Button {
    rect: Rect,
    pressed: bool,

    label: Option<Label>,
    state: Handle<ButtonState>,

    callback: Option<Box<dyn Fn(&mut Context) -> Transition>>,

    last_version: u64,
    dirty: bool,
}

pub struct ButtonState {
    pub icon: Option<Icon>,
    pub style: ButtonStyle,
}

impl Button {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0, 0, 0, 0),
            pressed: false,
            label: None,
            state: Handle::new(ButtonState {
                icon: None,
                style: ButtonStyle::default(),
            }),
            callback: None,
            last_version: 0,
            dirty: true,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        let mut label = Label::new(text)
            .text_align(TextAlign::Center)
            .vertical_align(VerticalAlign::Center);

        label.set_bounds(self.rect);
        self.label = Some(label);
        self
    }

    pub fn text_font(mut self, font: &'static Font) -> Self {
        if let Some(label) = self.label {
            self.label = Some(label.font(font));
        }
        self
    }

    pub fn icon(self, icon: Icon) -> Self {
        self.state.modify(|state| {
            state.icon = Some(icon);
        });
        self
    }

    pub fn style(self, style: ButtonStyle) -> Self {
        self.state.modify(|state| {
            state.style = style;
        });
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

    pub fn bounds(mut self, rect: Rect) -> Self {
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
        let state = self.state.get();
        let style = state.style;
        let icon = state.icon;

        let (background, light, dark) = match self.pressed {
            false => (style.background, style.light, style.dark),
            true => (
                style.pressed_background,
                style.pressed_light,
                style.pressed_dark,
            ),
        };

        renderer.draw_rounded_rect_3d(
            self.rect,
            style.radius,
            style.border_width,
            background,
            light,
            dark,
        );

        if let Some(label) = &self.label {
            label.render(renderer);
        }

        if let Some(icon) = icon {
            renderer.draw_icon(
                icon,
                self.rect.pad(style.icon_padding),
                style.background.invert(),
            );
        }
    }

    fn render_square_button(&self, renderer: &mut Renderer) {
        let style = self.state.get().style;

        let (background, light, dark) = match self.pressed {
            false => (style.background, style.light, style.dark),
            true => (
                style.pressed_background,
                style.pressed_light,
                style.pressed_dark,
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

    pub fn handle(&self) -> Handle<ButtonState> {
        self.state.clone()
    }
}

impl Widget for Button {
    fn update(&mut self, _dt: Duration) {
        let version = self.state.version();

        if version != self.last_version {
            self.last_version = version;
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
        let state = self.state.get();
        if state.style.radius > 0 {
            self.render_rounded_button(renderer);
        } else {
            self.render_square_button(renderer);
        }
    }
}
