use std::path::PathBuf;

use crate::music::track::Track;

pub struct MusicLibrary {
    root: PathBuf,
}

impl MusicLibrary {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn path(&self, song: &Track) -> PathBuf {
        self.root.join(&song.path)
    }
}
