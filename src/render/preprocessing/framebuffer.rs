extern crate gl;

use crate::Texture;

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

    pub fn attach_texture(&self, texture: &Texture, channel: gl::types::GLenum) {
        unsafe {

        }
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