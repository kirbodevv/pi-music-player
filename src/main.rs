mod cube;
mod platform;
mod renderer;

use std::thread;
use std::time::{Duration, Instant};

use cube::Cube;
use platform::Platform;
use renderer::Renderer;

#[cfg(feature = "desktop")]
use platform::desktop::Desktop;

#[cfg(feature = "raspberry")]
use platform::raspberry::Raspberry;

fn main() -> std::io::Result<()> {
    let mut renderer = Renderer::new();
    let mut cube = Cube::new();

    #[cfg(feature = "desktop")]
    let mut platform = Desktop::new()?;

    #[cfg(feature = "raspberry")]
    let mut platform = Raspberry::new()?;

    renderer.clear(0);

    if let Some(dirty) = renderer.take_dirty() {
        platform.present(&renderer, dirty)?;
    }

    let mut last = Instant::now();
    #[cfg(feature = "debug")]
    let mut fps_timer = Instant::now();
    #[cfg(feature = "debug")]
    let mut frames = 0u32;
    let mut max_present_ms = 0.0f64;

    const FRAME_TIME: Duration = Duration::from_micros(16_667);

    let mut next_frame = Instant::now();

    loop {
        if platform.should_close() {
            break;
        }

        platform.poll_events();

        let now = Instant::now();
        let dt = (now - last).as_secs_f32();
        last = now;

        cube.update(dt);
        cube.render(&mut renderer);

        if let Some(dirty) = renderer.take_dirty() {
            let present_start = Instant::now();

            platform.present(&renderer, dirty)?;

            let present_time = present_start.elapsed();
            let present_ms = present_time.as_secs_f64() * 1000.0;
            max_present_ms = max_present_ms.max(present_ms);

            #[cfg(feature = "debug")]
            {
                frames += 1;

                if fps_timer.elapsed() >= Duration::from_secs(1) {
                    println!(
                        "FPS: {} | dirty: {}x{} @ {},{} | present: {:.2} ms | max: {:.2} ms",
                        frames,
                        dirty.width,
                        dirty.height,
                        dirty.x,
                        dirty.y,
                        present_ms,
                        max_present_ms
                    );

                    max_present_ms = 0.0;

                    frames = 0;
                    fps_timer = Instant::now();
                }
            }
        }

        next_frame += FRAME_TIME;

        if let Some(remaining) = next_frame.checked_duration_since(Instant::now()) {
            thread::sleep(remaining);
        } else {
            // Если вывод занял слишком много времени —
            // не пытаемся "догнать" старые кадры.
            next_frame = Instant::now();
        }
    }

    Ok(())
}
