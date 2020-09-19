use image::{DynamicImage, GenericImageView, ImageError};

use crate::math::rect::Rect;
use crate::math::vect::Vect;

pub struct Texture {
    id: gl::types::GLuint,
    img: DynamicImage,
    size: Vect
}

impl Texture {
    pub fn default(path: &str) -> Result<Texture, ImageError> {
        Self::new(path, gl::NEAREST, gl::RGBA)
    }

    pub fn new(path: &str, mode: gl::types::GLenum, color: gl::types::GLenum) -> Result<Texture, ImageError> {
        let img = image::open(path)?.flipv();


        Ok(Self::from_img(img, mode, color))
    }

    pub fn from_img(img: DynamicImage ,mode: gl::types::GLenum, color: gl::types::GLenum) -> Texture {
        let mut id: gl::types::GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut id);

            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mode as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mode as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                color as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                color,
                gl::UNSIGNED_BYTE,
                img.to_bytes().as_ptr() as *const gl::types::GLvoid
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture{id, size: Vect::u32(img.width() , img.height()), img }
    }

    pub fn size(&self) -> Vect {
        self.size
    }

    pub fn frame(&self) -> Rect {
        Rect::from_vec(self.size)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, self.id as *const u32)
        }
    }
}