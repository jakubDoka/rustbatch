extern crate sdl2;

use std::ops::Deref;

use sdl2::{video, VideoSubsystem};

use crate::math::{mat, rgba};
use crate::math::mat::Mat;
use crate::math::vect::Vect;
use sdl2::video::GLContext;
use crate::render::buffer::Buffer;
use crate::render::batch::Batch;
use crate::math::rgba::RGBA;
use self::sdl2::EventPump;

static mut INITED: bool = false;

pub struct Window {
    buffer: Buffer,
    window: video::Window,
    camera: Mat,
    background_color: RGBA
}

impl Deref for Window {
    type Target = video::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl Window {

    pub fn new<F: FnOnce(&VideoSubsystem) -> video::Window>(gen: F) -> (Window, EventPump, GLContext) {
        Self::from_buffer(gen, || Buffer::default())
    }

    pub fn from_buffer<F, B>(gen: F, buffer: B) -> (Window, EventPump, GLContext)
        where F: FnOnce(&VideoSubsystem) -> video::Window,
              B: FnOnce() -> Buffer  {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = gen(&video_subsystem);

        let gl = window.gl_create_context().unwrap();
        let _fl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        (Window{window, buffer: buffer(), camera: mat::IM, background_color: rgba::BLACK}, sdl.event_pump().unwrap(), gl)
    }

    /*pub fn init(_: &video::Window, _: &GLContext, system: &VideoSubsystem) {
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

        Window{window, buffer, camera: mat::IM, background_color: rgba::BLACK}
    }*/

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

    pub fn set_background_color(&mut self, color: &RGBA) {
        self.background_color = color.clone();
    }


    pub fn set_camera(&mut self, mat: Mat) {
        self.camera = mat;
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(
                self.background_color[0],
                self.background_color[1],
                self.background_color[2],
                self.background_color[3],
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}