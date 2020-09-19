extern crate sdl2;

use std::ops::Deref;

use sdl2::{video, VideoSubsystem};

use crate::math::mat;
use crate::math::mat::Mat;
use crate::math::vect::Vect;
use sdl2::video::GLContext;
use crate::render::buffer::Buffer;
use crate::render::batch::Batch;

static mut INITED: bool = false;

pub struct Window {
    buffer: Buffer,
    window: video::Window,
    camera: Mat,
}

impl Deref for Window {
    type Target = video::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl Window {

    pub fn init(_: &video::Window, _: &GLContext, system: &VideoSubsystem) {
        unsafe {
            INITED = true;
        }
        let gl_attr = system.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let _gl = gl::load_with(|s| system.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn new(win: video::Window) -> Window {
        Self::from_buffer(win, Buffer::new())
    }

    pub fn from_buffer(window: video::Window, buffer: Buffer) -> Window {
        unsafe {
            if !INITED {
                panic!("you forgot to call Window::init()")
            }
        }

        Window{window, buffer, camera: mat::IM}
    }

    pub fn update(& self) {
        let (w, h) = self.size();
        unsafe {
            gl::Viewport(0, 0, w as i32, h as i32);
        }
        self.gl_swap_window();
    }

    pub fn draw(&self, batch: &Batch) {
        let (w, h) = self.size();

        batch.program.set_camera(self.camera);
        batch.program.set_view_size(Vect::new(w as f32, h as f32));
        batch.program.set_used();
        batch.texture.set_used();

        self.buffer.set_vertices_and_indices(&batch.vertices, &batch.indices);

        self.buffer.set_used();
        self.buffer.draw(batch.indices.len());
    }

    pub fn set_camera(&mut self, mat: Mat) {
        self.camera = mat;
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}