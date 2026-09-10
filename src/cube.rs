use std::f32::consts::PI;

use crate::renderer::{HEIGHT, Rect, Renderer, WIDTH};

#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn rotate_x(self, angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();

        Self {
            x: self.x,
            y: self.y * c - self.z * s,
            z: self.y * s + self.z * c,
        }
    }

    fn rotate_y(self, angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();

        Self {
            x: self.x * c + self.z * s,
            y: self.y,
            z: -self.x * s + self.z * c,
        }
    }

    fn rotate_z(self, angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();

        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
            z: self.z,
        }
    }
}

pub struct Cube {
    angle: f32,
    previous_bounds: Option<Rect>,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            angle: 0.0,
            previous_bounds: None,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.angle += dt * 1.5;

        if self.angle > PI * 2.0 {
            self.angle -= PI * 2.0;
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        // 1. Стираем старое положение куба.
        if let Some(old) = self.previous_bounds {
            renderer.clear_rect(old.expand(3), 0);
        }

        let vertices = [
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            Vec3 {
                x: 1.0,
                y: -1.0,
                z: -1.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: -1.0,
            },
            Vec3 {
                x: -1.0,
                y: 1.0,
                z: -1.0,
            },
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            Vec3 {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            },
        ];

        let edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];

        let mut projected = [(0i32, 0i32); 8];

        let mut min_x = WIDTH as i32;
        let mut min_y = HEIGHT as i32;
        let mut max_x = 0i32;
        let mut max_y = 0i32;

        for (i, vertex) in vertices.iter().enumerate() {
            let v = vertex
                .rotate_x(self.angle * 0.7)
                .rotate_y(self.angle)
                .rotate_z(self.angle * 0.3);

            let camera = 4.0;
            let scale = 110.0;

            let z = v.z + camera;

            let x = (v.x * scale / z) + WIDTH as f32 / 2.0;
            let y = (v.y * scale / z) + HEIGHT as f32 / 2.0;

            let x = x as i32;
            let y = y as i32;

            projected[i] = (x, y);

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        for &(a, b) in &edges {
            let (x0, y0) = projected[a];
            let (x1, y1) = projected[b];

            draw_line(renderer, x0, y0, x1, y1);
        }

        let bounds = Rect {
            x: min_x.max(0) as usize,
            y: min_y.max(0) as usize,
            width: (max_x - min_x).max(1) as usize,
            height: (max_y - min_y).max(1) as usize,
        }
        .expand(3);

        self.previous_bounds = Some(bounds);

        // На всякий случай явно помечаем область.
        renderer.mark_dirty(bounds);
    }
}

fn draw_line(renderer: &mut Renderer, mut x0: i32, mut y0: i32, x1: i32, y1: i32) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };

    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;

    let white = Renderer::rgb565(255, 255, 255);

    loop {
        renderer.pixel(x0, y0, white);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x0 += sx;
        }

        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
