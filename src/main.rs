mod platform;
mod renderer;
mod ui;

use std::thread;
use std::time::{Duration, Instant};

use platform::Platform;
use renderer::Renderer;

#[cfg(feature = "desktop")]
use platform::desktop::Desktop;

#[cfg(feature = "raspberry")]
use platform::raspberry::Raspberry;

fn main() -> std::io::Result<()> {
    let mut renderer = Renderer::new();
    let mut ui = ui::Ui::new();

    #[cfg(feature = "desktop")]
    let mut platform = Desktop::new()?;

    #[cfg(feature = "raspberry")]
    let mut platform = Raspberry::new()?;

    #[cfg(feature = "raspberry")]
    let mut touchscreen = platform::touchscreen::Touchscreen::open("/dev/input/event0")?;

    renderer.clear(0);

    if let Some(dirty) = renderer.take_dirty() {
        platform.present(&renderer, dirty)?;
    }

    const FRAME_TIME: Duration = Duration::from_micros(16_667);

    let mut next_frame = Instant::now();

    loop {
        if platform.should_close() {
            break;
        }

        #[cfg(feature = "raspberry")]
        for event in touchscreen.poll()? {
            ui.handle_input(event);
        }

        platform.poll_events();

        ui.render(&mut renderer);

        if let Some(dirty) = renderer.take_dirty() {
            platform.present(&renderer, dirty)?;
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
