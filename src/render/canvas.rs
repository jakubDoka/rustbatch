use crate::{Texture, Sprite, Batch, Mat, Vect, WHITE};
use crate::render::program::Program;
use crate::render::buffer::Buffer;
use crate::render::batch::Target;
use std::ops::Deref;
use crate::render::texture::{TextureSize, TextureConfig};
use crate::math::rgba::{RGBA, BLACK};

pub struct Canvas {
    framebuffer: gl::types::GLuint,
    batch: Batch,
    buffer: Buffer,
    drawer: Sprite,
    pub(crate) camera: Mat,
    background_color: RGBA
}

impl Target for Canvas {
    fn append(&mut self, data: &[f32], pattern: &[u32], vertex_size: u32, program: Option<&Program>, texture: Option<&Texture>, buffer: &Option<Buffer>) {


        self.bind();

        unsafe {
            gl::Viewport(0, 0, self.batch.texture.w, self.batch.texture.h);
        }

        match program {
            Some(p) => {
                p.bind();
                p.set_camera(self.camera.transform_from_window_space((self.batch.texture.w, self.batch.texture.h)));
                p.set_view_size(self.batch.texture.size());
            }
            None => panic!("Program mustn't be None. If you are using sprite to draw directly to \
            canvas use batch instead. Using sprites to draw is fundamentally ineffective so i decided \
            to not support it at all."),
        }

        if let Some(t) = texture {
            t.bind()
        }

        let buffer = match buffer {
            Some(t) => t,
            None => {
                if self.buffer.data_size as u32 != vertex_size {
                    panic!("incorrect vertex size, this canvas accepts only vertex size {}, but \
              you inputted vertex data with vertex size {}", self.buffer.data_size, vertex_size);
                }
                &self.buffer
            },
        };

        buffer.set_vertices_and_indices(data, pattern);
        buffer.bind();
        buffer.draw(pattern.len());

        Self::unbind()
    }


}

impl Canvas {
    #[inline]
    pub fn new(size: TextureSize) -> Self {
        Self::customized(size, Buffer::default(), Program::default(), TextureConfig::DEFAULT)
    }

    pub fn customized(size: TextureSize, buffer: Buffer, program: Program, config: TextureConfig) -> Self {
        let mut framebuffer: gl::types::GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
        }

        let tex = Texture::empty_texture(size, config);
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex.id(), 0);
        }
        assert!(unsafe {
            gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE
        });
        Self::unbind();
        Self {
            framebuffer,
            drawer: Sprite::new(tex.frame()),
            camera: Mat::IM,
            batch: Batch::customized(tex, program, None),
            buffer,
            background_color: BLACK,
        }
    }

    #[inline]
    pub fn set_program(&mut self, program: Program) {
        self.batch.program = program;
    }

    #[inline]
    pub fn set_buffer(&mut self, buffer: Buffer) {
        self.buffer = buffer;
    }

    #[inline]
    pub fn resize(&mut self, size: TextureSize) {
        self.batch.texture.resize_and_clear(size);
        self.batch.program.set_texture_size(self.batch.texture.size());
        self.drawer = Sprite::new(self.batch.texture.frame());
    }

    #[inline]
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);
        }
    }

    #[inline]
    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    /// set_background_color sets background color yey
    #[inline]
    pub fn set_background_color(&mut self, color: &RGBA) {
        self.background_color = color.clone();
    }

    /// set camera sets the matrix by witch are all shapes transformed so you can squeeze thinks
    /// however you like
    #[inline]
    pub fn set_camera_matrix(&mut self, mat: Mat) {
        self.camera = mat;
    }

    /// set_camera is more human readable way of setting viewport
    #[inline]
    pub fn set_camera(&mut self, position: Vect, zoom: f32) {
        self.camera = Mat::IM.moved(position.inverted()).scaled(Vect::ZERO, zoom);
    }

    pub fn redraw(&mut self, transform: &Mat, color: &RGBA) {
        self.batch.clear();
        self.drawer.draw_with_matrix(&mut self.batch, transform, color);
    }

    pub fn render(&mut self) {
        let (w, h) = self.size();

        self.batch.program.set_camera(Mat::IM);
        self.batch.program.set_view_size(vect!(w, h));
        self.batch.program.bind();
        self.batch.texture.bind();

        self.buffer.set_vertices_and_indices(&self.batch.data.vertices, &self.batch.data.indices);
        self.buffer.bind();
        self.buffer.draw(self.batch.data.indices.len());
    }

    #[inline]
    pub fn size(&self) -> (u32, u32){
        (self.batch.texture.w as u32, self.batch.texture.h as u32)
    }

    /// clear clears the window
    #[inline]
    pub fn clear(&self) {
        self.bind();
        super::clear(&self.background_color);
        Self::unbind();
    }
}