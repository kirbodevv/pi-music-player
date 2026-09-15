use std::path::{Path, PathBuf};

use crate::renderer::{Image, Scale};

const COVER_SIZE: u32 = 150;

pub struct ArtworkService {
    cached: Option<(PathBuf, Image)>,
}

impl ArtworkService {
    pub fn new() -> Self {
        Self { cached: None }
    }

    pub fn cover_for(&mut self, track_path: &Path) -> Image {
        let up_to_date = self
            .cached
            .as_ref()
            .is_some_and(|(cached_path, _)| cached_path == track_path);

        if !up_to_date {
            let cover_path = track_path.parent().map(|dir| dir.join("cover.jpg"));

            let image = match cover_path {
                Some(cover_path) => match Image::load(
                    &cover_path,
                    Scale::Exact {
                        width: COVER_SIZE,
                        height: COVER_SIZE,
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

            self.cached = Some((track_path.to_path_buf(), image));
        }

        let (_, image) = self.cached.as_ref().unwrap();
        clone_image(image)
    }
}

fn clone_image(image: &Image) -> Image {
    Image {
        width: image.width,
        height: image.height,
        pixels: image.pixels.clone(),
    }
}
