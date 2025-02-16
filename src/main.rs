use image::codecs::png::PngEncoder;
use image::{load_from_memory, ImageBuffer, Rgba};

use anyhow::anyhow;
use anyhow::Result;
use clap::{arg, command, Parser};
use image::ImageReader;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AsdfArgs {
    /// Input file path or read from stdin if not provided
    #[arg(short = 'i', long = "input", value_name = "INPUT")]
    input: Option<PathBuf>,

    /// Output file path or write to stdout if not provided
    #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// Processing direction ('h' for columns first, 'v' for rows first)
    #[arg(short = 'd', long = "direction", value_name = "DIRECTION")]
    direction: String,

    /// Sorting mode
    #[arg(short = 'm', long = "mode", value_name = "MODE")]
    mode: Mode,
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    White,
    Black,
    Bright,
    Dark,
}

impl std::str::FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "white" => Ok(Mode::White),
            "black" => Ok(Mode::Black),
            "bright" => Ok(Mode::Bright),
            "dark" => Ok(Mode::Dark),
            _ => Err(anyhow!(
                "Invalid mode. Must be one of: white, black, bright, dark"
            )),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = AsdfArgs::parse();

    // Read input
    let image_data = if let Some(input_path) = &args.input {
        ImageReader::open(input_path)?.decode()?
    } else {
        let stdin = std::io::stdin();

        let mut buffer = Vec::new();
        stdin.lock().read_to_end(&mut buffer)?;
        load_from_memory(buffer.as_slice())?
    };

    // Convert to rgba8
    let mut img = image_data.to_rgba8();

    // Process the image based on direction
    match args.direction.as_str() {
        "h" => {
            for x in 0..img.width() {
                process_column(&mut img, x, args.mode);
            }
            for y in 0..img.height() {
                process_row(&mut img, y, args.mode);
            }
        }
        "v" => {
            for y in 0..img.height() {
                process_row(&mut img, y, args.mode);
            }
            for x in 0..img.width() {
                process_column(&mut img, x, args.mode);
            }
        }
        _ => unreachable!(),
    }

    // Write output
    if let Some(output_path) = &args.output {
        img.save(output_path)?;
    } else {
        let stdout = std::io::stdout();
        let encoder = PngEncoder::new(stdout);
        img.write_with_encoder(encoder)?;
    }

    Ok(())
}

const WHITE_THRESHOLD: u32 = 0x123456u32;
const BLACK_THRESHOLD: u32 = 0x345678u32;
const BRIGHT_THRESHOLD: u8 = 127;
const DARK_THRESHOLD: u8 = 223;

fn pixel_value(pixel: &Rgba<u8>) -> u32 {
    let [r, g, b, _] = pixel.0;
    (r as u32) * (g as u32) * (b as u32)
}

fn brightness(pixel: &Rgba<u8>) -> u8 {
    let [r, g, b, _] = pixel.0;
    ((r as u16 + g as u16 + b as u16) / 3) as u8
}

fn should_sort(pixel: &Rgba<u8>, mode: Mode) -> bool {
    match mode {
        Mode::White => pixel_value(pixel) < WHITE_THRESHOLD,
        Mode::Black => pixel_value(pixel) > BLACK_THRESHOLD,
        Mode::Bright => brightness(pixel) > BRIGHT_THRESHOLD,
        Mode::Dark => brightness(pixel) < DARK_THRESHOLD,
    }
}

fn process_row(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, y: u32, mode: Mode) {
    let mut x = 0;
    while x < img.width() {
        while x < img.width() && !should_sort(img.get_pixel(x, y), mode) {
            x += 1;
        }
        let start = x;
        while x < img.width() && should_sort(img.get_pixel(x, y), mode) {
            x += 1;
        }
        let end = x;

        if start < end {
            let mut segment: Vec<_> = (start..end).map(|x| *img.get_pixel(x, y)).collect();
            segment.sort_by(|a, b| {
                let av = pixel_value(a);
                let bv = pixel_value(b);
                match mode {
                    Mode::White | Mode::Bright => av.cmp(&bv),
                    Mode::Black | Mode::Dark => bv.cmp(&av),
                }
            });

            for (i, pixel) in segment.into_iter().enumerate() {
                img.put_pixel(start + i as u32, y, pixel);
            }
        }
    }
}

fn process_column(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, mode: Mode) {
    let mut y = 0;
    while y < img.height() {
        while y < img.height() && !should_sort(img.get_pixel(x, y), mode) {
            y += 1;
        }
        let start = y;
        while y < img.height() && should_sort(img.get_pixel(x, y), mode) {
            y += 1;
        }
        let end = y;

        if start < end {
            let mut segment: Vec<_> = (start..end).map(|y| *img.get_pixel(x, y)).collect();
            segment.sort_by(|a, b| {
                let av = pixel_value(a);
                let bv = pixel_value(b);
                match mode {
                    Mode::White | Mode::Bright => av.cmp(&bv),
                    Mode::Black | Mode::Dark => bv.cmp(&av),
                }
            });

            for (i, &mut pixel) in segment.iter_mut().enumerate() {
                img.put_pixel(x, start + i as u32, pixel);
            }
        }
    }
}
