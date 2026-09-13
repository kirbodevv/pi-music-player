use std::io::Error;
use std::time::{Duration, Instant};
use std::{io, thread};

use crate::music::player::{AudioPlayer, PlayerState};
use crate::{music::mpd::MpdPlayer, platform::Platform, renderer::Renderer, ui::Ui};

pub struct App<P> {
    platform: P,
    renderer: Renderer,
    ui: Ui,
    next_frame: Instant,
    running: bool,
    player: MpdPlayer,
    last_player_update: Instant,
}

const FRAME_TIME: Duration = Duration::from_micros(16_667);

impl<P: Platform> App<P> {
    pub fn new(platform: P) -> Result<Self, Error> {
        Ok(Self {
            platform,
            renderer: Renderer::new(),
            ui: Ui::new(),
            next_frame: Instant::now(),
            running: true,
            player: MpdPlayer::connect("127.0.0.1:6600")
                .map_err(|e| Error::new(io::ErrorKind::Other, e))?,
            last_player_update: Instant::now(),
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
        for event in self.platform.poll_events() {
            let ui_event = self.ui.handle_input(event);

            let result = match ui_event {
                crate::ui::UiEvent::Player(action) => match action {
                    crate::ui::PlayerAction::Previous => self.player.previous(),

                    crate::ui::PlayerAction::PlayPause => {
                        let state = self
                            .player
                            .state()
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                        self.player.pause(state != PlayerState::Paused)
                    }

                    crate::ui::PlayerAction::Next => self.player.next(),
                },

                _ => Ok(()),
            };

            if let Err(e) = result {
                eprintln!("MPD error: {e}");
            }
        }

        if self.last_player_update.elapsed() >= Duration::from_millis(250) {
            self.last_player_update = Instant::now();

            match self.player.current_track() {
                Ok(track) => {
                    self.ui.set_current_track(track.as_ref(), &self.player);
                }

                Err(e) => {
                    eprintln!("MPD status error: {e}");
                }
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
