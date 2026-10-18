use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

use crate::renderer::{
    Image,
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

type ArtworkKey = (PathBuf, u32);

pub struct ArtworkService {
    cached: Option<(PathBuf, u32, Artwork)>,
    pending: Option<ArtworkKey>,
    delivered: Option<ArtworkKey>,
    request_tx: Sender<ArtworkKey>,
    result_rx: Receiver<(ArtworkKey, Artwork)>,
}

#[derive(Debug, Clone)]
pub struct Artwork {
    pub image: Image,
    pub theme: ArtworkTheme,
}

impl ArtworkService {
    pub fn new() -> Self {
        let (request_tx, request_rx) = channel::<ArtworkKey>();
        let (result_tx, result_rx) = channel::<(ArtworkKey, Artwork)>();

        thread::Builder::new()
            .name("artwork-loader".to_string())
            .spawn(move || artwork_loader_loop(request_rx, result_tx))
            .expect("failed to spawn artwork loader thread");

        Self {
            cached: None,
            pending: None,
            delivered: None,
            request_tx,
            result_rx,
        }
    }

    pub fn cover_for(&mut self, track_path: &Path, size: u32) -> Artwork {
        self.drain_results();

        let key = (track_path.to_path_buf(), size);
        let up_to_date = self
            .cached
            .as_ref()
            .is_some_and(|(cached_path, cached_size, _)| {
                cached_path == track_path && *cached_size == size
            });

        if !up_to_date {
            self.request(key);
        }

        match &self.cached {
            Some((_, _, artwork)) => artwork.clone(),
            None => Artwork {
                image: Image::default(),
                theme: ArtworkTheme::default(),
            },
        }
    }

    pub fn poll_cover(&mut self, track_path: &Path, size: u32) -> Option<Artwork> {
        self.drain_results();

        let key = (track_path.to_path_buf(), size);
        let up_to_date = self
            .cached
            .as_ref()
            .is_some_and(|(cached_path, cached_size, _)| {
                cached_path == track_path && *cached_size == size
            });

        if !up_to_date {
            self.request(key);
            return None;
        }

        if self.delivered.as_ref() == Some(&key) {
            return None;
        }

        self.delivered = Some(key);
        self.cached.as_ref().map(|(_, _, artwork)| artwork.clone())
    }

    fn request(&mut self, key: ArtworkKey) {
        let already_pending = self.pending.as_ref() == Some(&key);

        if !already_pending {
            self.pending = Some(key.clone());
            let _ = self.request_tx.send(key);
        }
    }

    fn drain_results(&mut self) {
        while let Ok((key, artwork)) = self.result_rx.try_recv() {
            if self.pending.as_ref() == Some(&key) {
                self.pending = None;
            }

            let (path, size) = key;
            self.cached = Some((path, size, artwork));
        }
    }
}

fn artwork_loader_loop(request_rx: Receiver<ArtworkKey>, result_tx: Sender<(ArtworkKey, Artwork)>) {
    while let Ok(mut key) = request_rx.recv() {
        while let Ok(newer_key) = request_rx.try_recv() {
            key = newer_key;
        }

        let artwork = load_artwork(&key.0, key.1);

        if result_tx.send((key, artwork)).is_err() {
            break;
        }
    }
}

fn load_artwork(track_path: &Path, size: u32) -> Artwork {
    let album_dir = track_path.parent();

    let image = match album_dir {
        Some(album_dir) => {
            let cover_path = album_dir.join("cover.jpg");
            let cache_path = album_dir.join("cover").join(format!("{size}.jpg"));

            match Image::load_with_thumbnail_cache(&cover_path, &cache_path, size) {
                Ok(image) => image,
                Err(error) => {
                    println!("Can't load cover for {}: {}", track_path.display(), error);
                    Image::default()
                }
            }
        }
        None => Image::default(),
    };

    let theme = theme_from_image(&image);

    Artwork { image, theme }
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
    coverage: f32,
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

    let step = 3;

    for y in (0..image.height).step_by(step) {
        for x in (0..image.width).step_by(step) {
            let index = y * image.width + x;

            if index >= image.pixels.len() {
                continue;
            }

            let color = Color::from_rgb565(image.pixels[index]);

            let (hue, saturation, value) = color.to_hsv();

            if value < 0.025 {
                continue;
            }

            if saturation < 0.035 {
                continue;
            }

            let saturation_weight = smoothstep(0.035, 0.35, saturation);

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
    let background_saturation = (saturation * 0.65).clamp(0.12, 0.70);
    let background_value = (0.12 + value * 0.18).clamp(0.12, 0.30);
    Color::hsv(hue, background_saturation, background_value)
}

fn make_light_color(hue: f32, saturation: f32, value: f32) -> Color {
    let light_saturation = (saturation * 0.75).clamp(0.20, 0.85);

    let light_value = (value + 0.15).clamp(0.45, 0.90);

    Color::hsv(hue, light_saturation, light_value)
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

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
