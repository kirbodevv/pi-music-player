use std::path::PathBuf;

use crate::music::track::Track;

pub struct Album {
    pub title: String,
    pub artist: String,
    pub cover: Option<PathBuf>,
    pub tracks: Vec<Track>,
}
