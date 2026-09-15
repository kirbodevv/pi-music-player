use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;

use mpd::error::Error;
use mpd::{Client, Song, State};

use crate::music::library::MusicLibrary;
use crate::music::player::{AudioPlayer, PlayerState};
use crate::music::track::Track;

pub struct MpdPlayer<S>
where
    S: Read + Write,
{
    client: Client<S>,
    pub library: MusicLibrary,
}

impl<S> MpdPlayer<S>
where
    S: Read + Write,
{
    pub fn connect(stream: S) -> Result<Self, Error> {
        let mut client = Client::new(stream)?;
        let music_directory = client.music_directory()?;

        Ok(Self {
            client,
            library: MusicLibrary::new(music_directory),
        })
    }
}

impl<S> AudioPlayer for MpdPlayer<S>
where
    S: Read + Write,
{
    type Error = mpd::error::Error;

    fn play(&mut self) -> Result<(), mpd::error::Error> {
        self.client.play()
    }

    fn pause(&mut self) -> Result<(), mpd::error::Error> {
        let state = self.state()?;
        self.client.pause(state != PlayerState::Paused)
    }

    fn stop(&mut self) -> Result<(), mpd::error::Error> {
        self.client.stop()
    }

    fn next(&mut self) -> Result<(), mpd::error::Error> {
        self.client.next()
    }

    fn previous(&mut self) -> Result<(), mpd::error::Error> {
        self.client.prev()
    }

    fn state(&mut self) -> Result<PlayerState, mpd::error::Error> {
        let status = self.client.status()?;

        Ok(match status.state {
            State::Play => PlayerState::Playing,
            State::Pause => PlayerState::Paused,
            State::Stop => PlayerState::Stopped,
        })
    }

    fn current_track(&mut self) -> Result<Option<Track>, mpd::error::Error> {
        let song = self.client.currentsong()?;

        Ok(song.map(track_from_mpd))
    }

    fn current_song_path(&mut self) -> Result<PathBuf, mpd::error::Error> {
        let song = self.client.currentsong()?;
        Ok(song
            .map(track_from_mpd)
            .map(|s| self.library.path(&s))
            .unwrap_or_default())
    }

    fn position(&mut self) -> Result<Duration, mpd::error::Error> {
        let status = self.client.status()?;

        Ok(status.elapsed.unwrap_or_default())
    }
}

fn track_from_mpd(song: Song) -> Track {
    Track {
        title: song.title.unwrap_or_default(),
        artist: song.artist.unwrap_or_default(),
        path: song.file.into(),
    }
}
