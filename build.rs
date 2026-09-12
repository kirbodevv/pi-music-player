use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use fontdue::Font;

const FONT_PATH: &str = "fonts/Inter_18pt-Bold.ttf";
const FONT_SIZES: &[u32] = &[12, 16, 20, 24, 32];

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed={FONT_PATH}");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
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

    // Cyrillic.
    for code in 0x0400u32..=0x04FFu32 {
        chars.push(char::from_u32(code).unwrap());
    }

    chars
}
