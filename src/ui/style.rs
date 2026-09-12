use crate::renderer::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct ButtonStyle {
    pub background: Color,
    pub light: Color,
    pub dark: Color,
    pub pressed_background: Color,
    pub pressed_light: Color,
    pub pressed_dark: Color,
    pub radius: usize,
    pub border_width: usize,
}

impl ButtonStyle {
    pub fn with_radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            background: Color::rgb(60, 60, 70),
            light: Color::rgb(100, 100, 115),
            dark: Color::rgb(30, 30, 38),

            pressed_background: Color::rgb(50, 50, 60),
            pressed_light: Color::rgb(70, 70, 82),
            pressed_dark: Color::rgb(25, 25, 32),

            radius: 0,
            border_width: 2,
        }
    }
}
