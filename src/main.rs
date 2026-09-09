use std::fs::OpenOptions;
use std::io;
use std::os::fd::AsRawFd;
use std::ptr;
use std::thread;
use std::time::Duration;

const WIDTH: usize = 480;
const HEIGHT: usize = 320;
const BYTES_PER_PIXEL: usize = 2;
const FB_SIZE: usize = WIDTH * HEIGHT * BYTES_PER_PIXEL;

#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

fn rotate(v: Vec3, ax: f32, ay: f32) -> Vec3 {
    let cx = ax.cos();
    let sx = ax.sin();

    let y = v.y * cx - v.z * sx;
    let z = v.y * sx + v.z * cx;

    let cy = ay.cos();
    let sy = ay.sin();

    let x = v.x * cy + z * sy;
    let z = -v.x * sy + z * cy;

    Vec3 { x, y, z }
}

fn project(v: Vec3) -> Point {
    let distance = 4.0;
    let scale = 220.0;

    let factor = scale / (v.z + distance);

    Point {
        x: v.x * factor + WIDTH as f32 / 2.0,
        y: -v.y * factor + HEIGHT as f32 / 2.0,
        z: v.z,
    }
}

fn rgb565(r: u8, g: u8, b: u8) -> u16 {
    ((r as u16 >> 3) << 11) | ((g as u16 >> 2) << 5) | (b as u16 >> 3)
}

fn clear(buffer: &mut [u16], color: u16) {
    buffer.fill(color);
}

fn draw_triangle(
    buffer: &mut [u16],
    zbuffer: &mut [f32],
    a: Point,
    b: Point,
    c: Point,
    color: u16,
) {
    let min_x = a.x.min(b.x).min(c.x).floor().max(0.0) as i32;
    let max_x = a.x.max(b.x).max(c.x).ceil().min((WIDTH - 1) as f32) as i32;

    let min_y = a.y.min(b.y).min(c.y).floor().max(0.0) as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil().min((HEIGHT - 1) as f32) as i32;

    let edge = |p1: Point, p2: Point, x: f32, y: f32| {
        (x - p1.x) * (p2.y - p1.y) - (y - p1.y) * (p2.x - p1.x)
    };

    let area = edge(a, b, c.x, c.y);

    if area.abs() < 0.001 {
        return;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;

            let w0 = edge(b, c, px, py);
            let w1 = edge(c, a, px, py);
            let w2 = edge(a, b, px, py);

            if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0) {
                let w0 = w0 / area;
                let w1 = w1 / area;
                let w2 = w2 / area;

                let z = a.z * w0 + b.z * w1 + c.z * w2;

                let index = y as usize * WIDTH + x as usize;

                if z < zbuffer[index] {
                    zbuffer[index] = z;
                    buffer[index] = color;
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    println!("Opening /dev/fb1...");

    let fb = OpenOptions::new().read(true).write(true).open("/dev/fb1")?;

    let fd = fb.as_raw_fd();

    let mapped = unsafe {
        libc::mmap(
            ptr::null_mut(),
            FB_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };

    if mapped == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    println!("Framebuffer mapped: {} bytes", FB_SIZE);
    println!("Starting cube...");

    let framebuffer = unsafe { std::slice::from_raw_parts_mut(mapped as *mut u16, WIDTH * HEIGHT) };

    let mut pixels = vec![0u16; WIDTH * HEIGHT];
    let mut zbuffer = vec![f32::INFINITY; WIDTH * HEIGHT];

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

    let faces = [
        ([0, 1, 2, 3], rgb565(0, 255, 0)),
        ([4, 7, 6, 5], rgb565(255, 0, 0)),
        ([0, 4, 5, 1], rgb565(0, 0, 255)),
        ([3, 2, 6, 7], rgb565(255, 255, 0)),
        ([1, 5, 6, 2], rgb565(255, 0, 255)),
        ([0, 3, 7, 4], rgb565(0, 255, 255)),
    ];

    let mut angle = 0.0f32;

    loop {
        clear(&mut pixels, rgb565(16, 16, 16));
        zbuffer.fill(f32::INFINITY);

        let ax = angle * 0.7;
        let ay = angle;

        let projected: Vec<Point> = vertices
            .iter()
            .map(|&v| project(rotate(v, ax, ay)))
            .collect();

        for (indices, color) in faces {
            let a = projected[indices[0]];
            let b = projected[indices[1]];
            let c = projected[indices[2]];
            let d = projected[indices[3]];

            draw_triangle(&mut pixels, &mut zbuffer, a, b, c, color);

            draw_triangle(&mut pixels, &mut zbuffer, a, c, d, color);
        }

        unsafe {
            ptr::copy_nonoverlapping(pixels.as_ptr(), framebuffer.as_mut_ptr(), pixels.len());
        }

        angle += 0.04;

        thread::sleep(Duration::from_millis(30));
    }
}
