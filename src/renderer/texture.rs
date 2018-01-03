use gl;

use ::images::{Image, ImageFormat};


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Texture {
    handle: u32
}


#[derive(Debug)]
pub enum TextureData<'a> {
    RGBA(&'a [u8])
}


#[derive(Debug, Copy, Clone)]
pub enum TextureFilter {
    Nearest = gl::NEAREST as isize,
    Linear = gl::LINEAR as isize
}

impl Texture {
    /// Create new
    pub fn new(width: u32, height: u32, pixels: TextureData) -> Texture {
        let mut handle = 0;
        unsafe {
            gl::GenTextures(1, &mut handle);

            use std::os::raw::c_void;
            let data = match pixels {
                TextureData::RGBA(slice) => {
                    slice.as_ptr() as *const c_void
                }
            };

            gl::BindTexture(gl::TEXTURE_2D, handle);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32,
                           width as i32, height as i32, 0,
                           gl::RGBA, gl::UNSIGNED_BYTE,data);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        Texture {
            handle
        }
    }


    /// Bind this texture
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
    }


    /// Set the min and mag filter for this texture
    pub fn set_min_mag_filter(&mut self, min_filter: TextureFilter, mag_filter: TextureFilter) {
        self.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
        }
    }

    /// Set the min and mag filter for this texture
    pub fn set_filter(&mut self, filter: TextureFilter) {
        self.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter as i32);
        }
    }
}


impl Default for Texture {
    fn default() -> Self {
        Texture::new(1, 1, TextureData::RGBA(&[255, 255, 255, 255]))
    }
}


impl From<Image> for Texture {
    fn from(image: Image) -> Self {
        let (width, height) = image.get_size();

        let data = match image.get_format() {
            ImageFormat::RGBA => TextureData::RGBA(image.as_bytes())
        };

        Texture::new(width, height, data)
    }
}
