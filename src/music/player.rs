use std::time::Duration;

use crate::music::track::Track;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Stopped,
    Playing,
    Paused,
}

pub trait AudioPlayer {
    type Error;

    fn play(&mut self) -> Result<(), Self::Error>;
    fn pause(&mut self, state: bool) -> Result<(), Self::Error>;
    fn stop(&mut self) -> Result<(), Self::Error>;
    fn next(&mut self) -> Result<(), Self::Error>;
    fn previous(&mut self) -> Result<(), Self::Error>;

    fn state(&mut self) -> Result<PlayerState, Self::Error>;
    fn current_track(&mut self) -> Result<Option<Track>, Self::Error>;
    fn position(&mut self) -> Result<Duration, Self::Error>;
}
