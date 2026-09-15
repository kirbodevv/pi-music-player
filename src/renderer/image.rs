use std::{fs, io, path::Path};

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
