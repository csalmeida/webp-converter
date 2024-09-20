# WebP Converter

A Rust library for converting image files to WebP format.

## Features

- Convert JPG, JPEG, PNG, and GIF images to WebP format
- Batch conversion of images in a directory
- Preserve directory structure during conversion
- Error handling for conversion process

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
webp_converter = "0.1.0"
```

## Usage

Basic usage:

```rust
use webp_converter::WebPConverter;

let converter = WebPConverter::new("path/to/source", "path/to/output");
converter.convert().unwrap();
```

Converting a specific directory:

```rust
use webp_converter::WebPConverter;
use std::path::Path;

let source = Path::new("/home/user/images");
let output = Path::new("/home/user/webp_images");
let converter = WebPConverter::new(source.to_str().unwrap(), output.to_str().unwrap());
converter.convert().unwrap();
```

Handling conversion errors:

```rust
use webp_converter::WebPConverter;

let converter = WebPConverter::new("path/to/source", "path/to/output");
match converter.convert() {
    Ok(_) => println!("All images converted successfully"),
    Err(e) => eprintln!("Error during conversion: {}", e),
}
```

## API

### `WebPConverter::new(source_dir: &str, output_dir: &str) -> Self`

Creates a new `WebPConverter` instance with the specified source and output directories.

### `WebPConverter::convert(&self) -> Result<(), Box<dyn std::error::Error>>`

Converts all supported image files in the source directory to WebP format and saves them in the output directory.

## License

This project is licensed under the MIT License.
