use clap::Parser;
use image::imageops::FilterType;
use image::GenericImageView;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// iResizer (Image Resizer)
///
/// Resize a single image or batch process a directory of images.
/// Supports JPG, PNG, BMP, and JPEG formats.
/// Resize using fixed dimensions (e.g., 800x600) or percentage (e.g., 50%).
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input image file or directory containing images
    #[arg(short, long)]
    input: PathBuf,

    /// Output image file or directory to save resized images
    #[arg(short, long)]
    output: PathBuf,

    /// Resize target: WIDTHxHEIGHT (e.g., 800x600) or percentage (e.g., 50%)
    #[arg(short, long)]
    size: String,

    /// Process directories recursively
    #[arg(short, long)]
    recursive: bool,
}

fn parse_size(size_str: &str, orig_width: u32, orig_height: u32) -> (u32, u32) {
    if size_str.ends_with('%') {
        let percent = size_str.trim_end_matches('%').parse::<f32>().expect("Invalid percentage");
        let scale = percent / 100.0;
        (
            (orig_width as f32 * scale) as u32,
            (orig_height as f32 * scale) as u32,
        )
    } else if let Some((w, h)) = size_str.split_once('x') {
        (
            w.parse::<u32>().expect("Invalid width"),
            h.parse::<u32>().expect("Invalid height"),
        )
    } else {
        panic!("Invalid size format. Use WIDTHxHEIGHT or PERCENT%");
    }
}

fn resize_image(input_path: &Path, output_path: &Path, size_str: &str, index: usize, total: usize) {
    println!("Processing {}/{}: {}", index, total, input_path.display());

    let original_size = fs::metadata(input_path).map(|m| m.len()).unwrap_or(0);

    let img = image::open(input_path).expect("Failed to open image");
    let (orig_width, orig_height) = img.dimensions();
    let (new_width, new_height) = parse_size(size_str, orig_width, orig_height);

    let resized = img.resize(new_width, new_height, FilterType::Lanczos3);
    resized.save(output_path).expect("Failed to save image");

    let resized_size = fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);

    println!(
        "Done: {} → {} ({} KB → {} KB)\n",
        input_path.file_name().unwrap().to_string_lossy(),
        output_path.file_name().unwrap().to_string_lossy(),
        original_size / 1024,
        resized_size / 1024
    );
}

fn process_directory(input_dir: &Path, output_dir: &Path, size_str: &str, recursive: bool) {
    let entries: Vec<_> = if recursive {
        WalkDir::new(input_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
            .filter(|e| {
                matches!(
                    e.path().extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase().as_str(),
                    "jpg" | "jpeg" | "png" | "bmp"
                )
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    } else {
        fs::read_dir(input_dir)
            .expect("Failed to read input directory")
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| {
                p.is_file()
                    && matches!(
                        p.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase().as_str(),
                        "jpg" | "jpeg" | "png" | "bmp"
                    )
            })
            .collect()
    };

    let total = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let relative_path = entry.strip_prefix(input_dir).unwrap();
        let output_path = output_dir.join(relative_path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create output subdirectory");
        }

        resize_image(entry, &output_path, size_str, i + 1, total);
    }
}


fn main() {
    println!(
        r#"
 ___ _____ ___   _____           _  
|_ _| ____|_ _| |_   _|__   ___ | |___  
 | ||  _|  | |    | |/ _ \ / _ \| / __|  
 | || |___ | |    | | (_) | (_) | \__ \  
|___|_____|___|   |_|\___/ \___/|_|___/  

iRezizer v0.1.1
Copyright (C) 2025 PT. Indonesia Epson Industry

A fast and flexible image resizer for a single file or all files in a directory.
Resize a single image or batch process all images in a directory.
Supports JPG, PNG, BMP, and JPEG formats.
Resize using fixed dimensions (e.g., 800x600) or percentage (e.g., 50%).
"#
    );

    let args = Args::parse();

    if args.input.is_file() {
        resize_image(&args.input, &args.output, &args.size, 1, 1);
    } else if args.input.is_dir() {
        fs::create_dir_all(&args.output).expect("Failed to create output directory");
        process_directory(&args.input, &args.output, &args.size, args.recursive);
    } else {
        panic!("Input path is neither a file nor a directory");
    }
}
