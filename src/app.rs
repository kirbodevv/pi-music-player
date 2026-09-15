use std::cell::RefCell;
use std::io::{Error, Read, Write};

use std::rc::Rc;
use std::time::{Duration, Instant};
use std::{io, thread};

use crate::{
    music::{mpd::MpdPlayer, service::MusicService},
    platform::Platform,
    renderer::Renderer,
    ui::Ui,
};

pub struct App<P, S>
where
    S: Read + Write + 'static,
{
    platform: P,
    renderer: Renderer,
    ui: Ui,
    last_update: Instant,
    next_frame: Instant,
    running: bool,
    music: MusicService<MpdPlayer<S>>,
}

const FRAME_TIME: Duration = Duration::from_micros(16_667);

impl<P: Platform, S: Read + Write + 'static> App<P, S> {
    pub fn new(platform: P, stream: S) -> Result<Self, Error> {
        let player = Rc::new(RefCell::new(
            MpdPlayer::connect(stream).map_err(|e| Error::new(io::ErrorKind::Other, e))?,
        ));

        let music = MusicService::new(player.clone());

        Ok(Self {
            platform,
            renderer: Renderer::new(),
            ui: Ui::new(player),
            last_update: Instant::now(),
            next_frame: Instant::now(),
            running: true,
            music,
        })
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
        let now = Instant::now();
        let dt = now.duration_since(self.last_update);
        self.last_update = now;

        self.music.poll();
        self.ui.update(dt);
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
