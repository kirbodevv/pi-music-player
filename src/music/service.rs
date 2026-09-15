use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::music::player::AudioPlayer;
use crate::music::track::Track;

const POLL_INTERVAL: Duration = Duration::from_millis(250);

pub struct MusicService<A: AudioPlayer> {
    player: Rc<RefCell<A>>,
    current_track: Option<Track>,
    current_song_path: Option<PathBuf>,
    last_poll: Instant,
}

impl<A: AudioPlayer> MusicService<A> {
    pub fn new(player: Rc<RefCell<A>>) -> Self {
        Self {
            player,
            current_track: None,
            current_song_path: None,
            // Force an immediate poll on the first call.
            last_poll: Instant::now() - POLL_INTERVAL,
        }
    }

    pub fn play_pause(&self) {
        let _ = self.player.borrow_mut().pause();
    }

    pub fn next(&self) {
        let _ = self.player.borrow_mut().next();
    }

    pub fn previous(&self) {
        let _ = self.player.borrow_mut().previous();
    }

    pub fn current_track(&self) -> Option<&Track> {
        self.current_track.as_ref()
    }

    pub fn current_song_path(&self) -> Option<&PathBuf> {
        self.current_song_path.as_ref()
    }

    pub fn poll(&mut self) {
        if self.last_poll.elapsed() < POLL_INTERVAL {
            return;
        }
        self.last_poll = Instant::now();

        let mut player = self.player.borrow_mut();

        if let Ok(Some(track)) = player.current_track() {
            let path = player.current_song_path().ok();
            self.current_track = Some(track);
            self.current_song_path = path;
        } else {
            self.current_track = None;
            self.current_song_path = None;
        }
    }
}
