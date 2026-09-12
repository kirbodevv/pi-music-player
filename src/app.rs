use std::time::{Duration, Instant};
use std::{io, thread};

use crate::platform::Platform;
use crate::{renderer::Renderer, ui::Ui};

pub struct App<P> {
    platform: P,
    renderer: Renderer,
    ui: Ui,
    next_frame: Instant,
    running: bool,
}

const FRAME_TIME: Duration = Duration::from_micros(16_667);

impl<P: Platform> App<P> {
    pub fn new(platform: P) -> Self {
        Self {
            platform,
            renderer: Renderer::new(),
            ui: Ui::new(),
            next_frame: Instant::now(),
            running: true,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        while self.running && !self.platform.should_close() {
            self.update()?;
            self.render()?;
            self.wait_for_next_frame();
        }

        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        for event in self.platform.poll_events() {
            self.ui.handle_input(event);
        }
        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        self.ui.render(&mut self.renderer);

        if let Some(dirty) = self.renderer.take_dirty() {
            self.platform.present(&self.renderer, dirty)?;
        }
        Ok(())
    }

    fn wait_for_next_frame(&mut self) {
        self.next_frame += FRAME_TIME;

        if let Some(remaining) = self.next_frame.checked_duration_since(Instant::now()) {
            thread::sleep(remaining);
        } else {
            self.next_frame = Instant::now();
        }
    }
}
