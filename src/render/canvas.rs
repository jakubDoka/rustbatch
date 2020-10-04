use crate::{Texture, Sprite, Mat, Vect};
use crate::render::program::Program;
use crate::render::buffer::Buffer;
use crate::render::batch::{Target, VertexData};
use crate::render::texture::{TextureSize, TextureConfig};
use crate::math::rgba::{RGBA, BLACK};

pub struct Canvas {
    framebuffer: gl::types::GLuint,
    data: VertexData,
    texture: Texture,
    program: Program,
    buffer: Buffer,
    drawer: Sprite,
    pub(crate) camera: Mat,
    background_color: RGBA
}

impl Target for Canvas {
    fn append(&mut self, data: &[f32], pattern: &[u32], vertex_size: u32, program: Option<&Program>, texture: Option<&Texture>, buffer: Option<&Buffer>) {


        self.bind();

        unsafe {
            gl::Viewport(0, 0, self.texture.w, self.texture.h);
        }

        match program {
            Some(p) => {
                p.bind();
                p.set_camera(self.camera.transform_from_window_space((self.texture.w, self.texture.h)));
                p.set_view_size(self.texture.size());
            }
            None => panic!("Program mustn't be None. If you are using sprite to draw directly to \
            canvas use batch instead. Using sprites to draw is fundamentally ineffective so i decided \
            to not support it at all."),
        }

        if let Some(t) = texture {
            t.bind()
        }

        let buffer = match buffer {
            Some(b) => b,
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

    /// customizes allows greater customization of canvas
    pub fn customized(size: TextureSize, buffer: Buffer, program: Program, config: TextureConfig) -> Self {
        let mut framebuffer: gl::types::GLuint = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
        }

        let texture = Texture::empty_texture(size, config);
        program.set_texture_size(texture.size());

        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture.id(), 0);
            assert!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE);
        }

        Self::unbind();

        Self {

            data: VertexData::custom(buffer.data_size as u32),
            drawer: Sprite::new(texture.frame()),
            camera: Mat::IM,
            background_color: BLACK,
            framebuffer,
            texture,
            program,
            buffer,
        }
    }

    #[inline]
    pub fn set_program(&mut self, program: Program) {
        self.program = program;
    }

    #[inline]
    pub fn set_buffer(&mut self, buffer: Buffer) {
        self.buffer = buffer;
    }

    /// resize resizes canvas and erases its content
    #[inline]
    pub fn resize(&mut self, size: TextureSize) {
        self.texture.resize_and_clear(size);
        self.program.set_texture_size(self.texture.size());
        self.drawer = Sprite::new(self.texture.frame());
    }

    /// bind binds the canvas so you can draw on it as you would on window
    #[inline]
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);
        }
    }

    /// unbind unbinds the current canvas. If you don't call this your draw calls will still affect
    /// this canvas and not the widow.
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

    ///size returns current size of canvas
    #[inline]
    pub fn size(&self) -> (u32, u32){
        (self.texture.w as u32, self.texture.h as u32)
    }

    /// clear clears the window
    #[inline]
    pub fn clear(&self) {
        self.bind();
        super::clear(&self.background_color);
        Self::unbind();
    }

    #[inline]
    pub fn draw<T: Target>(&mut self, other: &mut T, mat: &Mat, color: &RGBA) {
        self.data.clear();
        self.drawer.draw_with_matrix(&mut self.data, mat, color);
        other.append(&self.data.vertices, &self.data.indices, self.data.vertex_size, Some(&self.program), Some(&self.texture), Some(&self.buffer));
    }
}