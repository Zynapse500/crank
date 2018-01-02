
use lodepng;
use rgb::{RGBA, ComponentBytes};

use std::path::Path;


#[derive(Clone)]
pub struct Image {
    buffer: Vec<RGBA<u8>>,
    format: ImageFormat,
    width: u32,
    height: u32,
}


#[derive(Copy, Clone)]
pub enum ImageFormat {
    RGBA
}



impl Image {
    /// Load a png from drive and decode it
    pub fn load_png<P: AsRef<Path>>(path: P) -> Result<Image, String> {
        match lodepng::decode32_file(path) {
            Ok(image) => {
                Ok(Image {
                    buffer: image.buffer,
                    format: ImageFormat::RGBA,
                    width: image.width as u32,
                    height: image.height as u32,
                })
            }

            Err(e) => Err(format!("Failed to load image: {}", e))
        }
    }


    /// Decode bytes in a png format
    pub fn decode_png(bytes: &[u8]) -> Result<Image, String> {
        match lodepng::decode32(bytes) {
            Ok(image) => {
                Ok(Image {
                    buffer: image.buffer,
                    format: ImageFormat::RGBA,
                    width: image.width as u32,
                    height: image.height as u32,
                })
            }

            Err(e) => Err(format!("Failed to load image: {}", e))
        }
    }


    /// Get the size of the image
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Get the width of the image
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the height of the image
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Get the bytes in the buffer
    pub fn as_bytes(&self) -> &[u8] {
        self.buffer.as_bytes()
    }

    /// Get the format of the data in the image
    pub fn get_format(&self) -> ImageFormat {
        self.format
    }
}
