use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub path: PathBuf,
}
