extern crate gl;

use crate::Texture;
use crate::render::preprocessing::render_buffer::RenderBuffer;

pub struct FrameBuffer {
    id: gl::types::GLuint,
}

impl FrameBuffer {
    #[inline]
    pub fn new() -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut id);
        }
        Self{ id }
    }

    #[inline]
    pub fn call<F: Fn(gl::types::GLuint)>(&self, func: F)  {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            func(self.id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn attach_render_buffer(&self, buffer: &RenderBuffer, kind: gl::types::GLenum) {
        self.call(|_| unsafe {
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, kind, gl::RENDERBUFFER, buffer.id());
        });
    }

    pub fn attach_texture(&self, texture: &Texture, channel: gl::types::GLenum) {
        self.call(|_| unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, channel, gl::RENDERBUFFER, texture.id(), 0);
        });
    }

    pub fn is_ok(&self) -> bool {
        unsafe {
            gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE
        }
    }

}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &mut self.id);
        }
    }
}