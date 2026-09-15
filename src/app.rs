use std::cell::RefCell;
use std::io::{Error, Read, Write};

use std::rc::Rc;
use std::time::{Duration, Instant};
use std::{io, thread};

use crate::music::player::AudioPlayer;
use crate::ui::screen::ScreenEvent;
use crate::{music::mpd::MpdPlayer, platform::Platform, renderer::Renderer, ui::Ui};

pub struct App<P, S>
where
    S: Read + Write + 'static,
{
    platform: P,
    renderer: Renderer,
    ui: Ui,
    next_frame: Instant,
    running: bool,
    player: Rc<RefCell<MpdPlayer<S>>>,
}

const FRAME_TIME: Duration = Duration::from_micros(16_667);

impl<P: Platform, S: Read + Write + 'static> App<P, S> {
    pub fn new(platform: P, stream: S) -> Result<Self, Error> {
        let player = Rc::new(RefCell::new(
            MpdPlayer::connect(stream).map_err(|e| Error::new(io::ErrorKind::Other, e))?,
        ));

        Ok(Self {
            platform,
            renderer: Renderer::new(),
            ui: Ui::new(player.clone()),
            next_frame: Instant::now(),
            running: true,
            player,
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
        self.ui.update();
        for event in self.platform.poll_events() {
            let ui_event = self.ui.handle_input(event);

            let result = match ui_event {
                ScreenEvent::Player(action) => {
                    let mut player = self.player.borrow_mut();
                    match action {
                        crate::ui::PlayerAction::Previous => player.previous(),
                        crate::ui::PlayerAction::PlayPause => player.pause(),
                        crate::ui::PlayerAction::Next => player.next(),
                    }
                }

                _ => Ok(()),
            };

            if let Err(e) = result {
                eprintln!("MPD error: {e}");
            }
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
