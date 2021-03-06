use image::{DynamicImage, GenericImageView, ImageError, ImageBuffer, Pixel};

use crate::math::rect::Rect;
use crate::math::vect::Vect;
use std::path::Path;
use std::ptr::null;

/// TextureSize is for verifying users input.
pub struct TextureSize(i32, i32);

impl TextureSize {
    pub fn new(w: i32, h: i32) -> Self {
        let mut max = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut max);
        }
        if w > max || h > max {
            panic!("opengl supports only texture sizes up to {}x{}, you requested {}x{}", max, max, w, h);
        }
        Self(w, h)
    }
}

#[derive(Copy, Clone)]
pub struct TextureConfig {
    format: gl::types::GLenum,
    filtering: gl::types::GLenum,
}

impl TextureConfig {
    pub const DEFAULT: Self = TextureConfig{ filtering: gl::NEAREST, format: gl::RGBA };

    pub fn new(format: gl::types::GLenum, filtering: gl::types::GLenum) -> Self {
        Self{ format, filtering }
    }
}

/// Texture is wrapper for opengl texture object
/// its just an unsafe pointer with useful methods
pub struct Texture {
    clone: bool,
    id: gl::types::GLuint,
    config: TextureConfig,
    pub(crate) w: i32,
    pub(crate) h: i32,
}

impl Texture {
    pub const NONE: Self = Self{ id: 0, w: 0, h: 0 , config: TextureConfig{format: 0, filtering: 0}, clone: false};

    /// default creates what i consider default texture from provided path
    /// (no interpolation, alfa channel)
    #[inline]
    pub fn default(path: &str) -> Result<Texture, ImageError> {
        Self::new(path, TextureConfig::DEFAULT)
    }

    /// empty texture creates texture with all pixels set to 0.
    /// its useful mainly for canvas.
    #[inline]
    pub fn empty_texture(size: TextureSize, config: TextureConfig) -> Self {
        Self::from_raw_data(size, null(), config, false)
    }

    /// new creates new texture. for mode you have two options:
    /// - gl::NEAREST - it makes pixels visible
    /// - gl::LINEAR - it linearly interpolates between pixels and generally looks little ugly
    pub fn new<P: AsRef<Path>>(path: P, config: TextureConfig) -> Result<Texture, ImageError> {
        let img = image::open(path)?.flipv();

        Ok(Self::from_img(&img, config))
    }

    /// from_img returns texture from provided DynamicImage in case you want to do some pre processing
    /// on texture
    #[inline]
    pub fn from_img(img: &DynamicImage ,config: TextureConfig) -> Texture {
        Self::from_raw_data(TextureSize::new(img.width() as i32 ,img.height() as i32) , img.to_bytes().as_ptr(), config, true)
    }

    /// its just lower level function, it may come handy
    pub fn from_raw_data(size: TextureSize, bytes: *const u8, config: TextureConfig, mipmap: bool) -> Texture {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);

            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, config.filtering as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, config.filtering as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                config.format as i32,
                size.0,
                size.1,
                0,
                config.format,
                gl::UNSIGNED_BYTE,
                bytes as *const gl::types::GLvoid
            );
            if mipmap { gl::GenerateMipmap(gl::TEXTURE_2D); }
        }

        Texture{id, w: size.0, h: size.1 , config, clone: false}
    }

    /// resize_and_clear si here purely for resizing canvas textures, otherwise not very useful
    /// because it also erases the content of a texture.
    pub fn resize_and_clear(&mut self, size: TextureSize) {
        self.w = size.0;
        self.h = size.1;

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                self.config.format as i32,
                self.w,
                self.h,
                0,
                self.config.format,
                gl::UNSIGNED_BYTE,
                null(),
            );
        }


    }

    /// to image takes texture from gpu memory to back to you so you can save texture
    pub fn to_image<P: Pixel<Subpixel = u8> + 'static >(&self) -> Option<ImageBuffer<P, Vec<u8>>> {
        let mut buffer = vec![0u8 ;(self.w * self.h * P::CHANNEL_COUNT as i32) as usize];

        self.bind();

        unsafe {
            gl::GetTexImage(gl::TEXTURE_2D, 0, self.config.format, gl::UNSIGNED_BYTE, buffer.as_mut_ptr() as *mut gl::types::GLvoid);
        }

        image::ImageBuffer::from_raw(self.w as u32, self.h as u32 ,buffer)
    }

    #[inline]
    pub fn size(&self) -> Vect {
        vect!(self.w, self.h)
    }

    /// frame returns bounding rectangle of texture useful for sprite
    #[inline]
    pub fn frame(&self) -> Rect {
        Rect::from_vec(self.size())
    }

    /// id returns texture id. its a pointer to gl texture object
    #[inline]
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    /// set used uses ah texture
    #[inline]
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            clone: true,
            h: self.h,
            w: self.w,
            config: self.config
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if self.clone {
            return
        }
        unsafe {
            gl::DeleteTextures(1, &mut self.id)
        }
    }
}

#[cfg(test)]
mod tests {



   /* #[test]
    fn retrieve_test() {
        let (mut window, mut event_pump, _gl, _sdl, _video_subsystem)  = Window::new(|sys| sys.window("adsa", 400, 400).opengl().build().unwrap());

        let img = image::open("C:/Users/jakub/Documents/programming/rust/src/rustbatch/assets/logo.png").unwrap();
        let mut id: gl::types::GLuint = 0;
        let raw_size = (img.width(), img.height());
        unsafe {
            gl::GenTextures(1, &mut id);

            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                raw_size.0 as i32,
                raw_size.1 as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.to_bytes().as_ptr() as *const gl::types::GLvoid
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        //panic!("bn");
        //let f = ID{id};
        let texture = Texture{id, size: Vect::u32(raw_size.0 , raw_size.1), raw_size , format: gl::RGBA};

        /*let texture = Texture::from_img(
            img.clone(),
            gl::NEAREST,
            gl::RGBA
        );*/
        //panic!("bn");
        //let copy: ImageBuffer<Rgba<u8>, Vec<u8>> = texture.to_buffer::<Rgba<u8>>().unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize((img.width() * img.height()) as usize, 0u8);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::GetTexImage(gl::TEXTURE_2D, 0, gl::RGBA, gl::UNSIGNED_BYTE, buffer.as_ptr() as *mut gl::types::GLvoid);
        }

        let copy: ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::from_raw(img.width(), img.height(), buffer).unwrap();
        for (p, p1) in img.pixels().zip(copy.pixels()) {
            assert_eq!((p.2).0, p1.0);
        }
    }*/
}