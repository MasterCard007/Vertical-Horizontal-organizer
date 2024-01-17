
# Image Organizer Script in Rust

## Introduction
This Rust script is designed to organize images in a directory into 'Vertical' and 'Horizontal' folders based on their orientation. It's a handy tool for quickly sorting through numerous images.

## Prerequisites
- Rust programming language
- Cargo (Rust's package manager)
- `image` crate for Rust
- `rayon` crate for Rust
- `indicatif` crate for Rust

## Installation
1. Clone the repository or download the script.
2. Ensure Rust and Cargo are installed on your system.
3. Run `cargo build` to compile the script.

## Usage
### For Windows 11 Users
To make it easier for Windows 11 users, a batch file named `Cat_Dimension.bat` is provided. Follow these steps:
1. Place the `Cat_Dimension.bat` file in the same directory as your images.
2. Simply double-click on the `Cat_Dimension.bat` file.
3. The script will automatically sort the images into 'Vertical' and 'Horizontal' directories based on their orientation.

### For Other Users
Navigate to the directory containing your images and run:
```
cargo run
```
The script will automatically sort the images into 'Vertical' and 'Horizontal' directories.

## Contributing
Contributions to this script are welcome. Please ensure to follow best practices for coding in Rust and provide documentation for your changes.

## License
This script is released under the MIT License. For more details, see the LICENSE file.
