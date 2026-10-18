use std::{fs, io, path::Path, time::SystemTime};

use image::{DynamicImage, GenericImageView, imageops::FilterType};

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    Original,
    FixedWidth(u32),
    FixedHeight(u32),
    Exact { width: u32, height: u32 },
}

#[derive(Debug, Default, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u16>,
}

impl Image {
    fn from_rgba8(image: image::RgbaImage) -> Self {
        let (width, height) = image.dimensions();

        let mut pixels = Vec::with_capacity((width * height) as usize);

        for pixel in image.pixels() {
            let [r, g, b, _a] = pixel.0;

            let rgb565 = ((r as u16 >> 3) << 11) | ((g as u16 >> 2) << 5) | (b as u16 >> 3);

            pixels.push(rgb565);
        }

        Self {
            width: width as usize,
            height: height as usize,
            pixels,
        }
    }

    pub fn load<P: AsRef<Path>>(path: P, scale: Scale) -> io::Result<Self> {
        let data = fs::read(path)?;

        let image = image::load_from_memory(&data)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

        let image = scale_image(image, scale);
        let rgba = image.to_rgba8();

        Ok(Self::from_rgba8(rgba))
    }

    pub fn load_with_thumbnail_cache(
        source: &Path,
        cache_path: &Path,
        size: u32,
    ) -> io::Result<Self> {
        if thumbnail_cache_is_fresh(source, cache_path) {
            match Self::load(
                cache_path,
                Scale::Exact {
                    width: size,
                    height: size,
                },
            ) {
                Ok(image) => return Ok(image),
                Err(_) => {
                    // Cache file is missing/corrupt; fall through and regenerate it.
                }
            }
        }

        let data = fs::read(source)?;

        let image = image::load_from_memory(&data)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

        let resized = scale_image(
            image,
            Scale::Exact {
                width: size,
                height: size,
            },
        );

        save_thumbnail(&resized, cache_path);

        let rgba = resized.to_rgba8();

        Ok(Self::from_rgba8(rgba))
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

fn thumbnail_cache_is_fresh(source: &Path, cache_path: &Path) -> bool {
    let (Ok(source_meta), Ok(cache_meta)) = (fs::metadata(source), fs::metadata(cache_path)) else {
        return false;
    };

    let source_modified = source_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let cache_modified = cache_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);

    cache_modified >= source_modified
}

fn save_thumbnail(image: &DynamicImage, cache_path: &Path) {
    if let Some(parent) = cache_path.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            println!(
                "Can't create thumbnail cache dir {}: {}",
                parent.display(),
                error
            );
            return;
        }
    }

    if let Err(error) = image.to_rgb8().save(cache_path) {
        println!(
            "Can't write thumbnail cache {}: {}",
            cache_path.display(),
            error
        );
    }
}

fn scale_image(image: DynamicImage, scale: Scale) -> DynamicImage {
    let (width, height) = image.dimensions();

    let (new_width, new_height) = match scale {
        Scale::Original => (width, height),

        Scale::FixedWidth(new_width) => {
            let new_height = (height as u64 * new_width as u64 / width as u64) as u32;

            (new_width, new_height.max(1))
        }

        Scale::FixedHeight(new_height) => {
            let new_width = (width as u64 * new_height as u64 / height as u64) as u32;

            (new_width.max(1), new_height)
        }

        Scale::Exact { width, height } => (width, height),
    };

    if new_width == width && new_height == height {
        return image;
    }

    image.resize_exact(new_width, new_height, FilterType::Lanczos3)
}
