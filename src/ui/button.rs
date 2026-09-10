use crate::renderer::Renderer;

pub struct Button {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub color: u16,
}

impl Button {
    pub fn new(x: usize, y: usize, width: usize, height: usize, color: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.fill_rect(self.x, self.y, self.width, self.height, self.color);
    }
}
