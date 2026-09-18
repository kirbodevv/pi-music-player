use std::path::{Path, PathBuf};

use crate::renderer::{Image, Scale};

/// Cover size used by screens that don't have a specific requirement.
pub const DEFAULT_COVER_SIZE: u32 = 150;

/// Loads and caches track/album cover art from disk so screens don't need
/// to know where cover images physically come from (embedded artwork,
/// `cover.jpg`, a future remote source, ...), and don't re-read them from
/// disk on every poll.
pub struct ArtworkService {
    cached: Option<(PathBuf, u32, Image)>,
}

impl ArtworkService {
    pub fn new() -> Self {
        Self { cached: None }
    }

    /// Returns the cover art associated with the given track file path,
    /// looking for a `cover.jpg` next to it, scaled to `size x size`. The
    /// result is cached and only reloaded from disk when `track_path` or
    /// `size` changes; a placeholder image is returned when no cover is
    /// found.
    pub fn cover_for(&mut self, track_path: &Path, size: u32) -> Image {
        let up_to_date = self
            .cached
            .as_ref()
            .is_some_and(|(cached_path, cached_size, _)| {
                cached_path == track_path && *cached_size == size
            });

        if !up_to_date {
            let cover_path = track_path.parent().map(|dir| dir.join("cover.jpg"));

            let image = match cover_path {
                Some(cover_path) => match Image::load(
                    &cover_path,
                    Scale::Exact {
                        width: size,
                        height: size,
                    },
                ) {
                    Ok(image) => image,
                    Err(error) => {
                        println!("Can't load cover for {}: {}", track_path.display(), error);
                        Image::default()
                    }
                },
                None => Image::default(),
            };

            self.cached = Some((track_path.to_path_buf(), size, image));
        }

        let (_, _, image) = self.cached.as_ref().unwrap();
        image.clone()
    }
}
