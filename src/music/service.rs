use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::music::player::AudioPlayer;
use crate::music::track::Track;

/// Minimum time between polling the underlying player for fresh state.
const POLL_INTERVAL: Duration = Duration::from_millis(250);

/// The single concrete error type produced by the current `AudioPlayer`
/// implementation (MPD). Fixed here so that the player can be stored as a
/// trait object, which keeps `Context` (and therefore `Screen`/`Widget`)
/// free of any generic parameter tied to the connection type.
pub type PlayerError = mpd::error::Error;

pub type SharedPlayer = Rc<RefCell<dyn AudioPlayer<Error = PlayerError>>>;

/// Wraps an [`AudioPlayer`] and exposes the operations/state a UI screen
/// actually needs, without the screen having to know about the concrete
/// player implementation or about polling details.
pub struct MusicService {
    player: SharedPlayer,
    current_track: Option<Track>,
    current_song_path: Option<PathBuf>,
    last_poll: Instant,
}

impl MusicService {
    pub fn new(player: SharedPlayer) -> Self {
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

    /// Refreshes cached player state, throttled to `POLL_INTERVAL` so that
    /// callers can invoke this every frame without hammering the connection.
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
