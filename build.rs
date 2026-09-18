use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::Deserialize;

use fontdue::Font;

const FONT_PATH: &str = "fonts/Inter_18pt-Bold.ttf";
const FONT_SIZES: &[u32] = &[12, 16, 20, 24, 32];

const ICON_MANIFEST: &str = "assets/icons/manifest.toml";

#[derive(Debug, Deserialize)]
struct IconManifest {
    version: String,
    size: u32,
    icons: Vec<String>,
}

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed={FONT_PATH}");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    generate_fonts(&out_dir)?;
    generate_icons(&out_dir)?;

    Ok(())
}

// -----------------------------------------------------------------------------
// Fonts
// -----------------------------------------------------------------------------

fn generate_fonts(out_dir: &PathBuf) -> io::Result<()> {
    let font_data = fs::read(FONT_PATH).expect(&format!("failed to read {}", FONT_PATH));

    let font = Font::from_bytes(font_data, fontdue::FontSettings::default())
        .expect("failed to parse Inter-Regular.ttf");

    let output = out_dir.join("generated_font.rs");

    let mut file = File::create(&output)?;

    writeln!(file, "// AUTO-GENERATED FILE. DO NOT EDIT.")?;
    writeln!(file)?;

    for &size in FONT_SIZES {
        generate_font(&mut file, &font, size)?;
    }

    Ok(())
}

fn generate_font(file: &mut File, font: &Font, size: u32) -> io::Result<()> {
    let font_name = format!("FONT_{size}");

    writeln!(file, "pub static {font_name}: Font = Font {{")?;
    writeln!(file, "    size: {size},")?;
    writeln!(file, "    glyphs: &[")?;

    for ch in supported_characters() {
        let (metrics, bitmap) = font.rasterize(ch, size as f32);

        writeln!(file, "        Some(Glyph {{")?;

        writeln!(file, "            character: {:?},", ch)?;
        writeln!(file, "            width: {},", metrics.width)?;
        writeln!(file, "            height: {},", metrics.height)?;
        writeln!(file, "            bearing_x: {},", metrics.xmin)?;
        writeln!(file, "            bearing_y: {},", metrics.ymin)?;
        writeln!(file, "            advance: {:.4},", metrics.advance_width)?;

        write!(file, "            bitmap: &[")?;

        for (i, value) in bitmap.iter().enumerate() {
            if i % 32 == 0 {
                writeln!(file)?;
                write!(file, "                ")?;
            }

            write!(file, "{value}, ")?;
        }

        writeln!(file)?;
        writeln!(file, "            ],")?;
        writeln!(file, "        }}),")?;
    }

    writeln!(file, "    ],")?;
    writeln!(file, "}};")?;
    writeln!(file)?;

    Ok(())
}

fn supported_characters() -> Vec<char> {
    let mut chars = Vec::new();

    // ASCII.
    for code in 0x20u32..=0x7Eu32 {
        chars.push(char::from_u32(code).unwrap());
    }

    chars.extend(['•', '→', '←', '↑', '↓', '✓', '×', '−']);

    // Cyrillic.
    for code in 0x0400u32..=0x04FFu32 {
        chars.push(char::from_u32(code).unwrap());
    }

    chars
}

// -----------------------------------------------------------------------------
// Icons
// -----------------------------------------------------------------------------
fn generate_icons(out_dir: &Path) -> io::Result<()> {
    let manifest_data = fs::read_to_string(ICON_MANIFEST)?;

    let manifest: IconManifest = toml::from_str(&manifest_data).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed to parse {ICON_MANIFEST}: {error}"),
        )
    })?;

    if manifest.size == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "icon size must be greater than zero",
        ));
    }

    let cache_dir = Path::new("assets")
        .join("icons")
        .join("cache")
        .join(&manifest.version);

    fs::create_dir_all(&cache_dir)?;

    let output = out_dir.join("generated_icons.rs");

    let mut file = File::create(output)?;

    writeln!(file, "// AUTO-GENERATED FILE. DO NOT EDIT.")?;
    writeln!(file)?;

    writeln!(file, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]")?;
    writeln!(file, "pub enum Icon {{")?;

    for name in &manifest.icons {
        writeln!(file, "    {},", icon_variant(name))?;
    }

    writeln!(file, "}}")?;
    writeln!(file)?;

    writeln!(file, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(file, "pub struct IconData {{")?;
    writeln!(file, "    pub width: u8,")?;
    writeln!(file, "    pub height: u8,")?;
    writeln!(file, "    pub data: &'static [u8],")?;
    writeln!(file, "}}")?;
    writeln!(file)?;

    for name in &manifest.icons {
        let variant = icon_name(name);

        let svg = load_icon_svg(&manifest.version, name)?;

        let rgba = rasterize_svg(&svg, manifest.size)?;

        let mask = rgba_to_mask(&rgba);
        let packed = pack_mask(&mask);

        write_icon_data(&mut file, &variant, manifest.size, &packed)?;
    }

    writeln!(file, "pub fn icon_data(icon: Icon) -> &'static IconData {{")?;

    writeln!(file, "    match icon {{")?;

    for name in &manifest.icons {
        let icon_name = icon_name(&name);
        let variant = icon_variant(&name);

        writeln!(file, "        Icon::{variant} => &ICON_{icon_name},")?;
    }

    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    Ok(())
}

fn rasterize_svg(svg: &str, size: u32) -> io::Result<Vec<u8>> {
    let options = usvg::Options::default();

    let tree = usvg::Tree::from_str(svg, &options)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;

    let mut pixmap = tiny_skia::Pixmap::new(size, size)
        .ok_or_else(|| io::Error::other("failed to create pixmap"))?;

    let transform = tiny_skia::Transform::default();

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    Ok(pixmap.data().to_vec())
}

fn pack_mask(mask: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity((mask.len() + 1) / 2);

    for pair in mask.chunks(2) {
        let high = pair[0] & 0x0F;
        let low = pair.get(1).copied().unwrap_or(0) & 0x0F;

        result.push((high << 4) | low);
    }

    result
}

fn rgba_to_mask(rgba: &[u8]) -> Vec<u8> {
    rgba.chunks_exact(4)
        .map(|pixel| {
            let alpha = pixel[3];

            // 8 bit → 4 bit
            (alpha >> 4) & 0x0F
        })
        .collect()
}

fn load_icon_svg(version: &str, name: &str) -> io::Result<String> {
    let cache_dir = Path::new("assets/icons/cache").join(version);

    fs::create_dir_all(&cache_dir)?;

    let path = cache_dir.join(format!("{name}.svg"));

    if path.exists() {
        return fs::read_to_string(path);
    }

    let svg = download_icon(version, name)?;

    fs::write(&path, &svg)?;

    Ok(svg)
}

fn download_icon(version: &str, name: &str) -> io::Result<String> {
    let url =
        format!("https://raw.githubusercontent.com/lucide-icons/lucide/{version}/icons/{name}.svg");

    let response = ureq::get(&url)
        .call()
        .map_err(|error| io::Error::other(format!("failed to download {name}: {error}")))?;

    response
        .into_body()
        .read_to_string()
        .map_err(io::Error::other)
}

fn write_icon_data(file: &mut File, variant: &str, size: u32, data: &[u8]) -> io::Result<()> {
    writeln!(file, "pub static ICON_{variant}: IconData = IconData {{")?;

    writeln!(file, "    width: {size},")?;

    writeln!(file, "    height: {size},")?;

    write!(file, "    data: &[")?;

    for (i, byte) in data.iter().enumerate() {
        if i % 16 == 0 {
            writeln!(file)?;
            write!(file, "        ")?;
        }

        write!(file, "0x{byte:02X}, ")?;
    }

    writeln!(file)?;
    writeln!(file, "    ],")?;
    writeln!(file, "}};")?;
    writeln!(file)?;

    Ok(())
}

fn icon_variant(name: &str) -> String {
    name.split('-')
        .map(|part| {
            let mut chars = part.chars();

            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn icon_name(name: &str) -> String {
    name.replace('-', "_").to_uppercase()
}
