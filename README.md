# iRezizer

## Description

**iRezizer** is a fast and flexible image resizer for a single file or all files in a directory. It supports resizing JPG, PNG, BMP, and JPEG formats using fixed dimensions (e.g., `800x600`) or percentage (e.g., `50%`).

## Features

- Resize a single image or batch process all images in a directory
- Supports JPG, PNG, BMP, and JPEG formats
- Resize using fixed dimensions or percentage
- Recursive directory processing with `--recursive` argument
- Preserves directory structure in output
- Displays progress and file size information

## Installation

1. Install Rust
2. Clone this repository:
   ```bash
   git clone https://github.com/indonesiaepsonindustry-isp/irezizer.git
   cd irezizer
   ```
3. Build the project
   ```bash
   cargo build --release
   ```
4. Usage
   ```bash
   irezizer --input <input_path> --output <output_path> --size <WIDTHxHEIGHT|PERCENT> [--recursive]
   ```
## Examples
Resize a single image to 50%:
```bash
irezizer --input photo.jpg --output resized.jpg --size 50%
```
Resize all images in a directory to 800x600:

```bash
irezizer --input ./images --output ./resized --size 800x600
```
Resize all images recursively:
```bash
irezizer --input ./images --output ./resized --size 50% --recursive
```
## License
This project is licensed under the MIT License.

