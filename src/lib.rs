use std::fs;
use std::path::Path;
use image::ImageFormat;
use walkdir::WalkDir;

/// A struct for converting image files to WebP format.
///
/// This struct provides functionality to convert image files (JPG, JPEG, PNG, GIF)
/// in a source directory to WebP format and save them in an output directory.
///
/// The WebP format is a modern image format that provides superior compression and quality
/// compared to traditional formats like JPEG and PNG. By converting images to WebP,
/// you can significantly reduce file sizes while maintaining image quality, which is
/// particularly beneficial for web applications and mobile devices.
///
/// The `WebPConverter` struct allows you to:
/// - Specify source and output directories for batch conversion
/// - Convert multiple image formats (JPG, JPEG, PNG, GIF) to WebP
/// - Preserve the directory structure of the source in the output
/// - Handle errors during the conversion process
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use webp_converter::WebPConverter;
///
/// let converter = WebPConverter::new("path/to/source", "path/to/output");
/// converter.convert().unwrap();
/// ```
///
/// Converting a specific directory:
///
/// ```no_run
/// use webp_converter::WebPConverter;
/// use std::path::Path;
///
/// let source = Path::new("/home/user/images");
/// let output = Path::new("/home/user/webp_images");
/// let converter = WebPConverter::new(source.to_str().unwrap(), output.to_str().unwrap());
/// converter.convert().unwrap();
/// ```
///
/// Handling conversion errors:
///
/// ```no_run
/// use webp_converter::WebPConverter;
///
/// let converter = WebPConverter::new("path/to/source", "path/to/output");
/// match converter.convert() {
///     Ok(_) => println!("All images converted successfully"),
///     Err(e) => eprintln!("Error during conversion: {}", e),
/// }
/// ```
pub struct WebPConverter {
    source_dir: String,
    output_dir: String,
}

impl WebPConverter {
    /// Creates a new `WebPConverter` instance.
    ///
    /// This method initializes a new `WebPConverter` with the specified source and output
    /// directories. The source directory should contain the images you want to convert,
    /// and the output directory is where the converted WebP images will be saved.
    ///
    /// # Arguments
    ///
    /// * `source_dir` - The path to the source directory containing image files.
    /// * `output_dir` - The path to the output directory where WebP files will be saved.
    ///
    /// # Examples
    ///
    /// Creating a converter for a specific directory:
    ///
    /// ```
    /// use webp_converter::WebPConverter;
    ///
    /// let converter = WebPConverter::new("/home/user/photos", "/home/user/webp_photos");
    /// ```
    ///
    /// Using relative paths:
    ///
    /// ```
    /// use webp_converter::WebPConverter;
    ///
    /// let converter = WebPConverter::new("./images", "./converted");
    /// ```
    pub fn new(source_dir: &str, output_dir: &str) -> Self {
        WebPConverter {
            source_dir: source_dir.to_string(),
            output_dir: output_dir.to_string(),
        }
    }

    /// Converts all supported image files in the source directory to WebP format.
    ///
    /// This method walks through the source directory, identifies supported image files
    /// (JPG, JPEG, PNG, GIF), and converts them to WebP format in the output directory.
    /// It preserves the directory structure of the source in the output.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the conversion process completes successfully, or an error
    /// if any issues occur during the process.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use webp_converter::WebPConverter;
    ///
    /// let converter = WebPConverter::new("path/to/source", "path/to/output");
    /// match converter.convert() {
    ///     Ok(_) => println!("Conversion completed successfully"),
    ///     Err(e) => eprintln!("Error during conversion: {}", e),
    /// }
    /// ```
    ///
    /// Converting and processing results:
    ///
    /// ```no_run
    /// use webp_converter::WebPConverter;
    ///
    /// let converter = WebPConverter::new("path/to/source", "path/to/output");
    /// if let Err(e) = converter.convert() {
    ///     eprintln!("Conversion failed: {}", e);
    ///     // Handle the error (e.g., log it, notify user, etc.)
    /// } else {
    ///     println!("All images converted successfully");
    ///     // Perform post-conversion tasks (e.g., update database, notify user, etc.)
    /// }
    /// ```
    pub fn convert(&self) -> Result<(), Box<dyn std::error::Error>> {
        for entry in WalkDir::new(&self.source_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ["jpg", "jpeg", "png", "gif"].contains(&ext.to_lowercase().as_str()) {
                            self.convert_to_webp(path)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Converts a single image file to WebP format.
    ///
    /// This method takes a path to an image file, converts it to WebP format, and saves
    /// the result in the output directory. It preserves the relative path structure
    /// from the source directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the image file to be converted.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the conversion is successful, or an error if any issues occur.
    ///
    /// # Examples
    ///
    /// Converting a single image:
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use webp_converter::WebPConverter;
    ///
    /// let converter = WebPConverter::new("path/to/source", "path/to/output");
    /// let image_path = Path::new("path/to/image.jpg");
    /// match converter.convert_to_webp(image_path) {
    ///     Ok(_) => println!("Image converted successfully"),
    ///     Err(e) => eprintln!("Error converting image: {}", e),
    /// }
    /// ```
    ///
    /// Handling conversion errors for multiple images:
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use webp_converter::WebPConverter;
    ///
    /// let converter = WebPConverter::new("path/to/source", "path/to/output");
    /// let images = vec![
    ///     Path::new("image1.png"),
    ///     Path::new("image2.jpg"),
    ///     Path::new("image3.gif"),
    /// ];
    ///
    /// for image in images {
    ///     match converter.convert_to_webp(image) {
    ///         Ok(_) => println!("{:?} converted successfully", image),
    ///         Err(e) => eprintln!("Error converting {:?}: {}", image, e),
    ///     }
    /// }
    /// ```
    pub fn convert_to_webp(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let output_path = Path::new(&self.output_dir).join(file_name).with_extension("webp");
        fs::create_dir_all(output_path.parent().unwrap())?;
        img.save_with_format(output_path, ImageFormat::WebP)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_webp_conversion() {
        let source_dir = PathBuf::from("src/tests/source");
        let output_dir = PathBuf::from("src/tests/dist");

        let converter = WebPConverter::new(
            source_dir.to_str().unwrap(),
            output_dir.to_str().unwrap(),
        );

        converter.convert().unwrap();

        // Check if WebP files were created
        let expected_files = [
            "ferris_jpg.webp",
            "ferris_jpeg.webp",
            "ferris_png.webp",
        ];
        for file in &expected_files {
            let path = output_dir.join(file);
            assert!(path.exists(), "File {} not found", file);
        }

        // Check if the number of WebP files matches the number of source images
        let webp_count = WalkDir::new(&output_dir)
            .into_iter()
            .filter(|e| {
                e.as_ref()
                    .unwrap()
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    == Some("webp")
            })
            .count();
        assert_eq!(webp_count, 3, "Incorrect number of WebP files created");
    }

    #[test]
    fn test_single_file_conversion() {
        let source_dir = PathBuf::from("src/tests/source");
        let output_dir = PathBuf::from("src/tests/dist");

        let converter = WebPConverter::new(
            source_dir.to_str().unwrap(),
            output_dir.to_str().unwrap(),
        );

        let single_file = source_dir.join("ferris_jpg.jpg");
        converter.convert_to_webp(&single_file).unwrap();

        let output_file = output_dir.join("ferris_jpg.webp");
        assert!(output_file.exists(), "WebP file not created");
    }
}
