
use image;

use std::path::Path;


#[derive(Clone)]
pub struct Image {
    buffer: Vec<u8>,
    format: ImageFormat,
    width: u32,
    height: u32,
}


#[derive(Copy, Clone)]
pub enum ImageFormat {
    RGBA
}



impl Image {
    /// Load an image from drive and decode it
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Image, String> {

        match image::open(path) {
            Ok(image) => Ok(Image::from_dynamic(image)),

            Err(e) => Err(format!("Failed to load images: {}", e))
        }
    }


    /// Decode bytes
    pub fn decode(bytes: &[u8]) -> Result<Image, String> {
        match image::load_from_memory(bytes) {
            Ok(image) => Ok(Image::from_dynamic(image)),

            Err(e) => Err(format!("Failed to load images: {}", e))
        }
    }


    /// Create image from a dynamic image
    fn from_dynamic(image: image::DynamicImage) -> Image {
        let rgba = image.to_rgba();

        Image {
            format: ImageFormat::RGBA,
            width: rgba.width(),
            height: rgba.height(),
            buffer: rgba.into_raw(),
        }
    }


    /// Get the size of the images
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Get the width of the images
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the height of the images
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Get the bytes in the buffer
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }

    /// Get the format of the data in the images
    pub fn get_format(&self) -> ImageFormat {
        self.format
    }
}
