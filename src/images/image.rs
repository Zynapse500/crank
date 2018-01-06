
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


    /// Split an image into multiple smaller, evenly sized, images
    pub fn split_tiles(self, tiles_x: u32, tiles_y: u32) -> Vec<Image> {
        let mut tiles = Vec::new();

        // Size of the tiles
        let w = (self.width as f32 / tiles_x as f32).ceil() as u32;
        let h = (self.height as f32 / tiles_y as f32).ceil() as u32;

        // Create tiles
        for y in 0..tiles_y {
            for x in 0..tiles_x {
                tiles.push(self.extract_region(
                    x * w, y * h, w, h
                ));
            }
        }

        tiles
    }


    /// Extract a region of an image
    fn extract_region(&self, x: u32, y: u32, w: u32, h: u32) -> Image {
        let bytes_per_pixel = match self.format {
            ImageFormat::RGBA => 4
        };

        let mut image = Image {
            buffer: Vec::with_capacity((bytes_per_pixel * w * h) as usize),
            format: self.format,
            width: w,
            height: h,
        };

        for src_y in y..y+h {
            for src_x in x..x+w {
                // Add black pixels if we're out of bounds
                if src_x >= self.width || src_y >= self.height {
                    for _ in 0..bytes_per_pixel {
                        image.buffer.push(0);
                    }
                }

                let start_index = bytes_per_pixel * (src_x + src_y * self.width);

                for b in 0..bytes_per_pixel {
                    image.buffer.push(self.buffer[(start_index + b) as usize]);
                }
            }
        }

        image
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
        &self.buffer
    }

    /// Get the format of the data in the image
    pub fn get_format(&self) -> ImageFormat {
        self.format
    }
}
