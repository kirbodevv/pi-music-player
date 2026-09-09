mod framebuffer;
mod renderer;

use framebuffer::Framebuffer;
use renderer::{Renderer, rgb565};

use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting music player...");

    let mut fb = Framebuffer::open().expect("Cannot open /dev/fb1");

    let mut renderer = Renderer::new();

    let bg = rgb565(15, 15, 18);
    let white = rgb565(240, 240, 240);
    let gray = rgb565(100, 100, 105);
    let accent = rgb565(40, 180, 255);

    loop {
        renderer.clear(bg);

        renderer.rect(0, 0, 480, 45, rgb565(25, 25, 30));
        renderer.rect(20, 15, 160, 4, white);
        renderer.rect_outline(25, 65, 430, 125, gray);

        renderer.rect(45, 82, 90, 90, accent);
        renderer.rect(160, 90, 180, 6, white);
        renderer.rect(160, 110, 130, 5, gray);
        renderer.rect(160, 130, 90, 5, gray);

        renderer.rect(25, 215, 430, 6, rgb565(50, 50, 55));

        renderer.rect(25, 215, 170, 6, accent);

        renderer.rect_outline(55, 245, 100, 50, gray);

        renderer.rect_outline(190, 240, 100, 60, white);

        renderer.rect_outline(325, 245, 100, 50, gray);

        renderer.line(85, 270, 110, 258, white);
        renderer.line(85, 270, 110, 282, white);

        renderer.rect(232, 258, 7, 25, white);

        renderer.rect(250, 258, 7, 25, white);

        renderer.line(370, 258, 395, 270, white);
        renderer.line(370, 282, 395, 270, white);

        fb.present(&renderer.pixels);

        thread::sleep(Duration::from_millis(30));
    }
}
