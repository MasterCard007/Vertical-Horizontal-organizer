use image::GenericImageView;
use rayon::prelude::*;
use std::fs::{self, DirEntry};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    // Create directories if they don't exist
    fs::create_dir_all("Vertical").unwrap();
    fs::create_dir_all("Horizontal").unwrap();

    // Get all PNG files in the current directory
    let files: Vec<_> = fs::read_dir(".")
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().unwrap_or_default() == "png")
        .collect();

    // Initialize progress bar
    let bar = ProgressBar::new(files.len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));

    // Process files in parallel
    files.par_iter().for_each(|entry| {
        process_file(entry);
        bar.inc(1);
    });

    bar.finish();
}

fn process_file(entry: &DirEntry) {
    let path = entry.path();
    let img = image::open(&path).unwrap();
    let (width, height) = img.dimensions();

    if width > height {
        fs::rename(&path, Path::new("Horizontal").join(path.file_name().unwrap())).unwrap();
    } else if height > width {
        fs::rename(&path, Path::new("Vertical").join(path.file_name().unwrap())).unwrap();
    }
    // If dimensions are equal, do nothing
}