#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    Fixed(usize),
    Auto,
    Fill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutParams {
    pub width: Dimension,
    pub height: Dimension,
}

impl Default for LayoutParams {
    fn default() -> Self {
        Self {
            width: Dimension::Auto,
            height: Dimension::Auto,
        }
    }
}

impl LayoutParams {
    pub fn fixed(width: usize, height: usize) -> Self {
        Self {
            width: Dimension::Fixed(width),
            height: Dimension::Fixed(height),
        }
    }

    pub fn fill() -> Self {
        Self {
            width: Dimension::Fill,
            height: Dimension::Fill,
        }
    }

    pub fn auto() -> Self {
        Self::default()
    }

    pub fn width_fill() -> Self {
        Self {
            width: Dimension::Fill,
            height: Dimension::Auto,
        }
    }

    pub fn height_fill() -> Self {
        Self {
            width: Dimension::Auto,
            height: Dimension::Fill,
        }
    }
}
