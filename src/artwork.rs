use std::path::{Path, PathBuf};

use crate::renderer::{
    Image, Scale,
    color::{Color, normalize_hue},
};

pub const DEFAULT_COVER_SIZE: u32 = 150;

#[derive(Debug, Clone)]
pub struct ArtworkTheme {
    pub primary: Color,
    pub dark: Color,
    pub light: Color,
}

impl Default for ArtworkTheme {
    fn default() -> Self {
        Self {
            primary: Color::rgb(80, 80, 88),
            dark: Color::rgb(18, 18, 24),
            light: Color::rgb(120, 120, 128),
        }
    }
}

pub struct ArtworkService {
    cached: Option<(PathBuf, u32, Artwork)>,
}

#[derive(Debug, Clone)]
pub struct Artwork {
    pub image: Image,
    pub theme: ArtworkTheme,
}

impl ArtworkService {
    pub fn new() -> Self {
        Self { cached: None }
    }

    pub fn cover_for(&mut self, track_path: &Path, size: u32) -> Artwork {
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
            let theme = theme_from_image(&image);
            self.cached = Some((track_path.to_path_buf(), size, Artwork { image, theme }));
        }

        let (_, _, artwork) = self.cached.as_ref().unwrap();
        artwork.clone()
    }
}

#[derive(Debug, Clone, Copy)]
struct ColorSample {
    hue: f32,
    saturation: f32,
    value: f32,
    weight: f32,
}

#[derive(Debug, Clone, Copy)]
struct ColorCluster {
    hue: f32,
    saturation: f32,
    value: f32,

    /// Доля изображения, которую занимает этот кластер.
    coverage: f32,

    /// Вес цвета с учётом saturation/value.
    weight: f32,
}

fn theme_from_image(image: &Image) -> ArtworkTheme {
    if image.pixels.is_empty() || image.width == 0 || image.height == 0 {
        return ArtworkTheme::default();
    }

    let samples = collect_samples(image);

    if samples.is_empty() {
        return ArtworkTheme::default();
    }

    let clusters = build_clusters(&samples);

    if clusters.is_empty() {
        return ArtworkTheme::default();
    }

    let cluster = choose_dominant_cluster(&clusters);

    let primary = Color::hsv(cluster.hue, cluster.saturation, cluster.value);

    let dark = make_dark_background(cluster.hue, cluster.saturation, cluster.value);

    let light = make_light_color(cluster.hue, cluster.saturation, cluster.value);

    ArtworkTheme {
        primary,
        dark,
        light,
    }
}

// -----------------------------------------------------------------------------
// Sampling
// -----------------------------------------------------------------------------

fn collect_samples(image: &Image) -> Vec<ColorSample> {
    let mut samples = Vec::new();

    /*
     * Нам не нужен каждый пиксель.
     *
     * Для cover 150x150 это всего 5625 пикселей,
     * но step=3 уже достаточно хорошо описывает картинку.
     */
    let step = 3;

    for y in (0..image.height).step_by(step) {
        for x in (0..image.width).step_by(step) {
            let index = y * image.width + x;

            if index >= image.pixels.len() {
                continue;
            }

            let color = Color::from_rgb565(image.pixels[index]);

            let (hue, saturation, value) = color.to_hsv();

            /*
             * Чёрные пиксели не помогают определить hue.
             */
            if value < 0.025 {
                continue;
            }

            /*
             * Очень слабые цвета тоже практически бесполезны
             * для определения атмосферы.
             */
            if saturation < 0.035 {
                continue;
            }

            /*
             * Чем насыщеннее цвет, тем сильнее он влияет.
             *
             * При этом не позволяем saturation полностью
             * доминировать над количеством пикселей.
             */
            let saturation_weight = smoothstep(0.035, 0.35, saturation);

            /*
             * Слишком тёмные цвета не должны иметь большой вес,
             * но полностью их исключать тоже не хочется.
             */
            let brightness_weight = 0.35 + value * 0.65;

            let weight = saturation_weight * brightness_weight;

            samples.push(ColorSample {
                hue,
                saturation,
                value,
                weight,
            });
        }
    }

    samples
}

// -----------------------------------------------------------------------------
// Clustering
// -----------------------------------------------------------------------------

fn build_clusters(samples: &[ColorSample]) -> Vec<ColorCluster> {
    /*
     * Делим hue на 36 сегментов по 10 градусов.
     *
     * Это намного дешевле K-means и для cover работает отлично.
     */
    const HUE_BUCKETS: usize = 36;

    let mut hue_weight = [0.0f32; HUE_BUCKETS];

    let mut saturation_sum = [0.0f32; HUE_BUCKETS];
    let mut value_sum = [0.0f32; HUE_BUCKETS];

    let mut raw_count = [0u32; HUE_BUCKETS];

    let mut total_weight = 0.0f32;

    for sample in samples {
        let bucket = ((sample.hue / 360.0) * HUE_BUCKETS as f32).floor() as usize;

        let bucket = bucket.min(HUE_BUCKETS - 1);

        hue_weight[bucket] += sample.weight;

        saturation_sum[bucket] += sample.saturation * sample.weight;

        value_sum[bucket] += sample.value * sample.weight;

        raw_count[bucket] += 1;

        total_weight += sample.weight;
    }

    if total_weight <= f32::EPSILON {
        return Vec::new();
    }

    /*
     * Объединяем соседние hue buckets.
     *
     * Например:
     *
     * 350° + 0° + 10°
     *
     * должны считаться одним красным кластером.
     */
    let mut merged = Vec::new();

    let mut visited = [false; HUE_BUCKETS];

    for start in 0..HUE_BUCKETS {
        if visited[start] || hue_weight[start] <= 0.0 {
            continue;
        }

        let mut total = 0.0f32;
        let mut weighted_hue = 0.0f32;
        let mut weighted_saturation = 0.0f32;
        let mut weighted_value = 0.0f32;

        for offset in -1i32..=1 {
            let bucket =
                ((start as i32 + offset + HUE_BUCKETS as i32) % HUE_BUCKETS as i32) as usize;

            if visited[bucket] {
                continue;
            }

            let weight = hue_weight[bucket];

            if weight <= 0.0 {
                continue;
            }

            visited[bucket] = true;

            let center_hue = (bucket as f32 + 0.5) * (360.0 / HUE_BUCKETS as f32);

            total += weight;

            weighted_hue += center_hue * weight;

            weighted_saturation += saturation_sum[bucket];

            weighted_value += value_sum[bucket];
        }

        if total <= f32::EPSILON {
            continue;
        }

        let hue = weighted_hue / total;
        let saturation = weighted_saturation / total;
        let value = weighted_value / total;

        let coverage = total / total_weight;

        merged.push(ColorCluster {
            hue: normalize_hue(hue),
            saturation,
            value,
            coverage,
            weight: total,
        });
    }

    merged
}

// -----------------------------------------------------------------------------
// Dominant color selection
// -----------------------------------------------------------------------------

fn choose_dominant_cluster(clusters: &[ColorCluster]) -> ColorCluster {
    clusters
        .iter()
        .copied()
        .filter(|cluster| cluster.coverage >= 0.015)
        .max_by(|a, b| {
            score_cluster(a)
                .partial_cmp(&score_cluster(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or_else(|| {
            clusters
                .iter()
                .copied()
                .max_by(|a, b| {
                    a.weight
                        .partial_cmp(&b.weight)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap()
        })
}

fn score_cluster(cluster: &ColorCluster) -> f32 {
    let saturation_score = smoothstep(0.10, 0.70, cluster.saturation);
    let coverage_score = cluster.coverage.sqrt();
    let darkness_score = 1.0 - smoothstep(0.65, 1.0, cluster.value);
    saturation_score * 0.55 + coverage_score * 0.30 + darkness_score * 0.15
}

// -----------------------------------------------------------------------------
// Color generation
// -----------------------------------------------------------------------------

fn make_dark_background(hue: f32, saturation: f32, value: f32) -> Color {
    let background_saturation = clamp(saturation * 0.65, 0.12, 0.70);
    let background_value = clamp(0.12 + value * 0.18, 0.12, 0.30);
    Color::hsv(hue, background_saturation, background_value)
}

fn make_light_color(hue: f32, saturation: f32, value: f32) -> Color {
    let light_saturation = clamp(saturation * 0.75, 0.20, 0.85);

    let light_value = clamp(value + 0.15, 0.45, 0.90);

    Color::hsv(hue, light_saturation, light_value)
}

// -----------------------------------------------------------------------------
// HSV → RGB
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

fn smoothstep(edge0: f32, edge1: f32, value: f32) -> f32 {
    if value <= edge0 {
        return 0.0;
    }

    if value >= edge1 {
        return 1.0;
    }

    let t = (value - edge0) / (edge1 - edge0);

    t * t * (3.0 - 2.0 * t)
}
