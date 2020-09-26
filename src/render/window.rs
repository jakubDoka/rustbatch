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
use self::sdl2::{EventPump, Sdl};
use crate::math::rect::Rect;

/// Window core feature of this library, everything starts here. Only after creating window you can
/// safely use other parts of render module because window also calls gl::load_with() witch makes
/// functions accessible. Little design decision is that (0, 0) is in the middle of screen by default
/// it makes managing gale viewport lot nicer in my opinion. Window also implements Deref to deref
/// inner sdl window.
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
    /// new creates new window. Cosure is just to deffer window creation for internal reasons.
    /// function also returns event pump from witch you can poll events like key presses and mouse
    /// moves. Third return value is simply here because it cannot be dropped otherwise window
    /// would not render.
    ///
    /// # Example
    /// ```
    /// use rustbatch::Window;
    ///
    /// let (window, event_pump, _gl, _sdl, _video_subsystem) = Window::new(|sys| {
    ///     sys.window("Title", 1000, 600)
    ///     .opengl()
    ///     .build()
    ///     .unwrap()
    /// });
    /// ```
    pub fn new<F: FnOnce(&VideoSubsystem) -> video::Window>(gen: F) -> (Window, EventPump, GLContext, Sdl, VideoSubsystem) {
        Self::from_buffer(gen, || Buffer::default())
    }

    /// from_buffer creates window with custom buffer. Remember that changing vertex structure
    /// requires also batch with custom program
    pub fn from_buffer<F, B>(gen: F, buffer: B) -> (Window, EventPump, GLContext, Sdl, VideoSubsystem)
        where F: FnOnce(&VideoSubsystem) -> video::Window,
              B: FnOnce() -> Buffer  {
        let sdl = sdl2::init().expect("You probably did not set up your project correctly go to \
        crates documentation, you can find all answers there.");

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

        (Window{window, buffer: buffer(), camera: Mat::IM, background_color: rgba::BLACK}, sdl.event_pump().unwrap(), gl, sdl, video_subsystem)
    }

    /// update updates window, just call it at the end
    pub fn update(& self) {
        let (w, h) = self.size();
        unsafe {
            gl::Viewport(0, 0, w as i32, h as i32);
        }
        self.gl_swap_window();
    }

    /// draw takes batch and draws it to screen. this function modifies uniforms of vertex shader
    pub fn draw(&self, batch: &Batch) {
        let (w, h) = self.size();

        batch.program.set_camera(self.camera.transform_from_window_space((w, h)));
        batch.program.set_view_size(Vect::u32(w, h));
        batch.program.set_used();
        batch.texture.set_used();

        self.buffer.set_vertices_and_indices(&batch.vertices, &batch.indices);
        self.buffer.set_used();
        self.buffer.draw(batch.indices.len());
    }

    /// get_viewport_rect returns rectangle that whole screen fits in and that is even if you
    /// rotate camera. Useful when you don't want to draw sprites that are foo screen
    pub fn get_viewport_rect(&self) -> Rect {
        let (mut w,mut h) = self.size();
        let w = w as i32 /2;
        let h = h as i32 /2;

        let mut corners: [Vect; 4] = [
            Vect::i32(-w, -h),
            Vect::i32(w, -h),
            Vect::i32(w, h),
            Vect::i32(-w, h),
        ];

        for i in 0..corners.len() {
            corners[i] = self.camera.unprj(corners[i]);
        }

        Rect::bounds_for(&corners)
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

    /// clear clears the window
    #[inline]
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