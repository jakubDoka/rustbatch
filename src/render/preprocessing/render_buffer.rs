
pub struct RenderBuffer {
    id: gl::types::GLuint
}

impl RenderBuffer {
    pub fn new() -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut id);
        }

        Self{ id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.id);
        }
    }



    pub fn call<F: Fn(gl::types::GLuint)>(&self, func: F) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.id);
            func(self.id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
    }

    pub fn reserve(&self, w: i32, h: i32, precision: gl::types::GLenum) {
        self.call(|_| unsafe {
            gl::RenderbufferStorage(gl::RENDERBUFFER, precision, w, h)
        });
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for RenderBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &mut self.id)
        }
    }
}