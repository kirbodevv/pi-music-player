use crate::renderer::{Rect, color::Color};

use super::Renderer;

impl Renderer {
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        for y in rect.y..rect.y + rect.height {
            for x in rect.x..rect.x + rect.width {
                self.pixel(x, y, color);
            }
        }
    }

    pub fn fill_rounded_rect(&mut self, rect: Rect, radius: usize, color: Color) {
        if rect.width == 0 || rect.height == 0 {
            return;
        }

        let radius = radius.min(rect.width / 2).min(rect.height / 2);

        let r = radius as i32;

        let left = rect.x as i32;
        let top = rect.y as i32;
        let right = (rect.x + rect.width - 1) as i32;
        let bottom = (rect.y + rect.height - 1) as i32;

        for y in top..=bottom {
            for x in left..=right {
                if is_inside_rounded_rect(x, y, left, top, right, bottom, r) {
                    self.pixel(x as usize, y as usize, color);
                }
            }
        }
    }

    pub fn draw_rounded_rect_3d(
        &mut self,
        rect: Rect,
        radius: usize,
        border: usize,
        background: Color,
        light: Color,
        dark: Color,
    ) {
        if rect.width == 0 || rect.height == 0 {
            return;
        }

        let radius = radius.min(rect.width / 2).min(rect.height / 2);

        let border = border.min(rect.width / 2).min(rect.height / 2);

        // Сначала вся поверхность.
        self.fill_rounded_rect(rect, radius, background);

        if border == 0 {
            return;
        }

        let left = rect.x as i32;
        let top = rect.y as i32;
        let right = (rect.x + rect.width - 1) as i32;
        let bottom = (rect.y + rect.height - 1) as i32;

        let r = radius as i32;

        for y in top..=bottom {
            for x in left..=right {
                if !is_inside_rounded_rect(x, y, left, top, right, bottom, r) {
                    continue;
                }

                let inner_left = left + border as i32;
                let inner_top = top + border as i32;
                let inner_right = right - border as i32;
                let inner_bottom = bottom - border as i32;

                let inner_radius = (radius as i32 - border as i32).max(0);

                let inside_inner = is_inside_rounded_rect(
                    x,
                    y,
                    inner_left,
                    inner_top,
                    inner_right,
                    inner_bottom,
                    inner_radius,
                );

                if inside_inner {
                    continue;
                }

                let dist_top = y - top;
                let dist_left = x - left;
                let dist_bottom = bottom - y;
                let dist_right = right - x;

                let min_light = dist_top.min(dist_left);
                let min_dark = dist_bottom.min(dist_right);

                let color = if min_light <= min_dark { light } else { dark };

                self.pixel(x as usize, y as usize, color);
            }
        }
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        if rect.width == 0 || rect.height == 0 {
            return;
        }

        for x in rect.x..rect.x + rect.width {
            self.pixel(x, rect.y, color);
            self.pixel(x, rect.y + rect.height - 1, color);
        }

        for y in rect.y..rect.y + rect.height {
            self.pixel(rect.x, y, color);
            self.pixel(rect.x + rect.width - 1, y, color);
        }
    }

    pub fn line(&mut self, mut x0: usize, mut y0: usize, x1: usize, y1: usize, color: Color) {
        let dx = (x1 as isize - x0 as isize).abs();
        let sx = if x0 < x1 { 1 } else { -1 };

        let dy = -(y1 as isize - y0 as isize).abs();
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.pixel(x0, y0, color);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;

            if e2 >= dy {
                err += dy;
                x0 = (x0 as isize + sx) as usize;
            }

            if e2 <= dx {
                err += dx;
                y0 = (y0 as isize + sy) as usize;
            }
        }
    }
}

fn is_inside_rounded_rect(
    x: i32,
    y: i32,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    radius: i32,
) -> bool {
    if radius == 0 {
        return true;
    }

    let corner_x = if x < left + radius {
        left + radius
    } else if x > right - radius {
        right - radius
    } else {
        x
    };

    let corner_y = if y < top + radius {
        top + radius
    } else if y > bottom - radius {
        bottom - radius
    } else {
        y
    };

    let dx = x - corner_x;
    let dy = y - corner_y;

    dx * dx + dy * dy <= radius * radius
}
