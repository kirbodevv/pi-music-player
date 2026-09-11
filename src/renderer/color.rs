#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn gray(value: u8) -> Self {
        Self::rgb(value, value, value)
    }

    pub const fn black() -> Self {
        Self::rgb(0, 0, 0)
    }

    pub const fn white() -> Self {
        Self::rgb(255, 255, 255)
    }

    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);

    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    pub const CYAN: Self = Self::rgb(0, 255, 255);
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);

    pub const GRAY: Self = Self::rgb(128, 128, 128);
    pub const LIGHT_GRAY: Self = Self::rgb(192, 192, 192);
    pub const DARK_GRAY: Self = Self::rgb(64, 64, 64);

    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);

    pub const fn to_rgb565(self) -> u16 {
        let r = (self.r as u16 >> 3) & 0x1F;
        let g = (self.g as u16 >> 2) & 0x3F;
        let b = (self.b as u16 >> 3) & 0x1F;

        (r << 11) | (g << 5) | b
    }

    pub const fn from_rgb565(value: u16) -> Self {
        let r = ((value >> 11) & 0x1F) as u8;
        let g = ((value >> 5) & 0x3F) as u8;
        let b = (value & 0x1F) as u8;

        let r = (r << 3) | (r >> 2);
        let g = (g << 2) | (g >> 4);
        let b = (b << 3) | (b >> 2);

        Self::rgb(r, g, b)
    }

    pub const fn with_alpha(self, alpha: u8) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    pub const fn alpha(self) -> u8 {
        self.a
    }

    pub const fn is_transparent(self) -> bool {
        self.a == 0
    }

    pub const fn is_opaque(self) -> bool {
        self.a == 255
    }

    pub fn blend_over(self, background: Self) -> Self {
        if self.a == 255 {
            return Self::rgb(self.r, self.g, self.b);
        }

        if self.a == 0 {
            return background;
        }

        let alpha = self.a as u16;
        let inv_alpha = 255 - alpha;

        let r = (self.r as u16 * alpha + background.r as u16 * inv_alpha) / 255;
        let g = (self.g as u16 * alpha + background.g as u16 * inv_alpha) / 255;
        let b = (self.b as u16 * alpha + background.b as u16 * inv_alpha) / 255;

        Self::rgb(r as u8, g as u8, b as u8)
    }

    pub fn lighten(self, amount: u8) -> Self {
        Self::rgb(
            self.r.saturating_add(amount),
            self.g.saturating_add(amount),
            self.b.saturating_add(amount),
        )
        .with_alpha(self.a)
    }

    pub fn darken(self, amount: u8) -> Self {
        Self::rgb(
            self.r.saturating_sub(amount),
            self.g.saturating_sub(amount),
            self.b.saturating_sub(amount),
        )
        .with_alpha(self.a)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);

        let r = self.r as f32 + (other.r as f32 - self.r as f32) * t;
        let g = self.g as f32 + (other.g as f32 - self.g as f32) * t;
        let b = self.b as f32 + (other.b as f32 - self.b as f32) * t;
        let a = self.a as f32 + (other.a as f32 - self.a as f32) * t;

        Self::rgba(
            r.round() as u8,
            g.round() as u8,
            b.round() as u8,
            a.round() as u8,
        )
    }

    pub const fn opaque(self) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: 255,
        }
    }
}

impl From<u16> for Color {
    fn from(value: u16) -> Self {
        Self::from_rgb565(value)
    }
}

impl From<Color> for u16 {
    fn from(color: Color) -> Self {
        color.to_rgb565()
    }
}
