use image::{DynamicImage, GenericImageView, ImageError};

use crate::math::rect::Rect;
use crate::math::vect::Vect;
use std::path::Path;

/// Texture is wrapper for opengl texture object
/// its just an unsafe pointer with useful methods
#[derive(Clone)]
pub struct Texture {
    id: gl::types::GLuint,
    size: Vect
}

impl Texture {
    /// default creates what i consider default texture from provided path
    /// (no interpolation, alfa channel)
    #[inline]
    pub fn default(path: &str) -> Result<Texture, ImageError> {
        Self::new(path, gl::NEAREST, gl::RGBA)
    }

    /// new creates new texture. for mode you have two options:
    /// - gl::NEAREST - it makes pixels visible
    /// - gl::LINEAR - it linearly interpolates between pixels and generally looks little ugly
    pub fn new<P: AsRef<Path>>(path: P, mode: gl::types::GLenum, color: gl::types::GLenum) -> Result<Texture, ImageError> {
        let img = image::open(path)?.flipv();

        Ok(Self::from_img(&img, mode, color))
    }

    /// from_img returns texture from provided DynamicImage in case you want to do some pre processing
    /// on texture
    pub fn from_img(img: &DynamicImage ,mode: gl::types::GLenum, color: gl::types::GLenum) -> Texture {
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

        Texture{id, size: Vect::u32(img.width() , img.height()) }
    }

    pub fn size(&self) -> Vect {
        self.size
    }

    /// frame returns bounding rectangle of texture useful for sprite
    pub fn frame(&self) -> Rect {
        Rect::from_vec(self.size)
    }

    /// id returns texture id. its a pointer to gl texture object
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    /// set used uses ah texture
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