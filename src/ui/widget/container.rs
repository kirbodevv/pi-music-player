use std::time::Duration;

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{Rect, Renderer, Size},
    ui::{
        screen::Transition,
        widget::{ContainerStyle, Dimension, Handle, LayoutParams, Widget},
    },
};

#[derive(Debug, Clone, Copy, Default)]
pub enum ContainerDirection {
    #[default]
    Vertical,
    Horizontal,
}

pub struct Child {
    widget: Box<dyn Widget>,
    layout: LayoutParams,
}

pub struct Container {
    rect: Rect,
    children: Vec<Child>,
    direction: ContainerDirection,
    spacing: usize,
    padding: usize,
    state: Handle<ContainerState>,
    last_version: u64,
    dirty: bool,
}

#[derive(Default)]
pub struct ContainerState {
    pub style: ContainerStyle,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            rect: Rect::default(),
            children: Vec::new(),
            direction: ContainerDirection::default(),
            spacing: 0,
            padding: 0,
            state: Handle::default(),
            last_version: 0,
            dirty: true,
        }
    }
}

impl Container {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            ..Self::default()
        }
    }

    pub fn with_child<W: Widget + 'static>(mut self, widget: W, layout: LayoutParams) -> Self {
        self.children.push(Child {
            widget: Box::new(widget),
            layout,
        });
        self.layout();
        self
    }

    pub fn direction(mut self, direction: ContainerDirection) -> Self {
        self.direction = direction;
        self.layout();
        self
    }

    pub fn spacing(mut self, spacing: usize) -> Self {
        self.spacing = spacing;
        self.layout();
        self
    }

    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self.layout();
        self
    }

    pub fn style(self, style: ContainerStyle) -> Self {
        self.state.modify(|state| state.style = style);
        self
    }

    fn layout(&mut self) {
        match self.direction {
            ContainerDirection::Vertical => self.layout_vertical(),
            ContainerDirection::Horizontal => self.layout_horizontal(),
        }
    }

    fn layout_vertical(&mut self) {
        let inner_height = self.rect.height.saturating_sub(self.padding * 2);

        let spacing_total = self
            .spacing
            .saturating_mul(self.children.len().saturating_sub(1));

        let available_height = inner_height.saturating_sub(spacing_total);

        let mut fixed_height = 0;
        let mut fill_count = 0;

        for child in &self.children {
            match child.layout.height {
                Dimension::Fixed(height) => {
                    fixed_height += height;
                }

                Dimension::Auto => {
                    fixed_height += child.widget.preferred_size().height;
                }

                Dimension::Fill => {
                    fill_count += 1;
                }
            }
        }

        let remaining_height = available_height.saturating_sub(fixed_height);

        let fill_height = if fill_count > 0 {
            remaining_height / fill_count
        } else {
            0
        };

        let mut y = self.rect.y + self.padding;

        for child in &mut self.children {
            let height = match child.layout.height {
                Dimension::Fixed(height) => height,

                Dimension::Auto => child.widget.preferred_size().height,

                Dimension::Fill => fill_height,
            };

            let width = match child.layout.width {
                Dimension::Fixed(width) => width,

                Dimension::Auto | Dimension::Fill => {
                    self.rect.width.saturating_sub(self.padding * 2)
                }
            };

            child.widget.set_bounds(Rect {
                x: self.rect.x + self.padding,
                y,
                width,
                height,
            });

            y += height + self.spacing;
        }
    }

    fn layout_horizontal(&mut self) {
        let inner_width = self.rect.width.saturating_sub(self.padding * 2);

        let spacing_total = self
            .spacing
            .saturating_mul(self.children.len().saturating_sub(1));

        let available_width = inner_width.saturating_sub(spacing_total);

        let mut fixed_width = 0;
        let mut fill_count = 0;

        for child in &self.children {
            match child.layout.width {
                Dimension::Fixed(width) => {
                    fixed_width += width;
                }

                Dimension::Auto => {
                    fixed_width += child.widget.preferred_size().width;
                }

                Dimension::Fill => {
                    fill_count += 1;
                }
            }
        }

        let remaining_width = available_width.saturating_sub(fixed_width);

        let fill_width = if fill_count > 0 {
            remaining_width / fill_count
        } else {
            0
        };

        let mut x = self.rect.x + self.padding;

        for child in &mut self.children {
            let preferred_size = child.widget.preferred_size();

            let width = match child.layout.width {
                Dimension::Fixed(width) => width,

                Dimension::Auto => preferred_size.width,

                Dimension::Fill => fill_width,
            };

            let height = match child.layout.height {
                Dimension::Fixed(height) => height,

                Dimension::Auto => preferred_size.height,

                Dimension::Fill => self.rect.height.saturating_sub(self.padding * 2),
            };

            child.widget.set_bounds(Rect {
                x,
                y: self.rect.y + self.padding,
                width,
                height,
            });

            x += width + self.spacing;
        }
    }

    pub fn handle(&self) -> Handle<ContainerState> {
        self.state.clone()
    }
}

impl Widget for Container {
    fn layout_params(&self) -> LayoutParams {
        LayoutParams::default()
    }

    fn preferred_size(&self) -> Size {
        Size {
            width: self.rect.width,
            height: self.rect.height,
        }
    }

    fn bounds(&self) -> Rect {
        self.rect
    }

    fn set_bounds(&mut self, rect: Rect) {
        self.rect = rect;
        self.layout();
        self.dirty = true;
    }

    fn handle_input(&mut self, event: &InputEvent, ctx: &mut Context) -> Transition {
        for child in self.children.iter_mut().rev() {
            let event = child.widget.handle_input(event, ctx);

            if event != Transition::None {
                return event;
            }
        }

        Transition::None
    }

    fn render(&self, renderer: &mut Renderer) {
        let bounds = self.bounds();

        let repaint_all = self.dirty;

        let style = self.state.get().style;
        renderer.with_clip(bounds, |renderer| {
            if repaint_all {
                if let Some(background) = style.background {
                    if style.radius > 0 {
                        renderer.fill_rounded_rect(bounds, style.radius, background);
                    } else {
                        renderer.fill_rect(bounds, background);
                    }
                }
            }

            for child in &self.children {
                if repaint_all || child.widget.is_dirty() {
                    child.widget.render(renderer);
                }
            }
        });
    }

    fn update(&mut self, dt: Duration) {
        let version = self.state.version();

        if version != self.last_version {
            self.last_version = version;
            self.dirty = true;
        }
        for child in &mut self.children {
            if self.dirty {
                child.widget.mark_dirty();
            }
            child.widget.update(dt);
        }
    }

    fn is_dirty(&self) -> bool {
        self.dirty || self.children.iter().any(|child| child.widget.is_dirty())
    }

    fn clear_dirty(&mut self) {
        self.dirty = false;

        for child in &mut self.children {
            child.widget.clear_dirty();
        }
    }

    fn mark_dirty(&mut self) {
        self.dirty = true;

        for child in &mut self.children {
            child.widget.mark_dirty();
        }
    }
}
