mod platform;
mod renderer;

use renderer::Renderer;

fn main() {
    let mut renderer = Renderer::new();
    let mut platform = platform::Platform::new();

    while platform.is_running() {
        renderer.clear(renderer::rgb565(16, 16, 16));

        // ВЕСЬ UI здесь / позже вынесем в ui.rs
        renderer.rect(0, 0, 480, 60, renderer::rgb565(32, 32, 32));

        renderer.rect(20, 80, 440, 150, renderer::rgb565(24, 24, 24));

        renderer.rect(20, 245, 440, 8, renderer::rgb565(48, 48, 48));

        renderer.rect(20, 245, 180, 8, renderer::rgb565(0, 170, 255));

        renderer.rect(70, 275, 70, 30, renderer::rgb565(48, 48, 48));

        renderer.rect(205, 270, 70, 40, renderer::rgb565(64, 64, 64));

        renderer.rect(340, 275, 70, 30, renderer::rgb565(48, 48, 48));

        platform.present(renderer.pixels());
    }
}
