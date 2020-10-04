extern crate sdl2;

use std::ops::Deref;
use sdl2::{video, VideoSubsystem};
use crate::math::mat::Mat;
use crate::math::vect::Vect;
use sdl2::video::GLContext;
use crate::render::buffer::Buffer;
use crate::render::batch::Target;
use self::sdl2::{EventPump, Sdl};
use crate::math::rect::Rect;
use crate::render::canvas::Canvas;
use crate::render::texture::TextureSize;
use crate::{WHITE, Texture};
use crate::render::program::Program;

/// Window core feature of this library, everything starts here. Only after creating window you can
/// safely use other parts of render module because window also calls gl::load_with() witch makes
/// functions accessible. Little design decision is that (0, 0) is in the middle of screen by default
/// it makes managing gale viewport lot nicer in my opinion. Window also implements Deref to deref
/// inner sdl window.
pub struct Window {
    pub canvas: Canvas,
    window: video::Window,
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

    /// from_buffer creates window with custom buffer. Remember that changing vertex structure
    /// requires also batch with custom program
    pub fn new<F, B>(gen: F) -> (Window, EventPump, GLContext, Sdl, VideoSubsystem)
        where F: FnOnce(&VideoSubsystem) -> video::Window,
              B: FnOnce() -> Buffer  {
        let sdl = sdl2::init().expect("You probably did not set up your project correctly go to \
        crates documentation, you can find all answers there.");

        let video_subsystem = sdl.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 0);

        let window = gen(&video_subsystem);

        let gl = window.gl_create_context().unwrap();
        let _fl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        let (w, h) = window.size();

        (Window{canvas: Canvas::new(TextureSize::new(w as i32, h as i32)) ,window}, sdl.event_pump().unwrap(), gl, sdl, video_subsystem)
    }

    /// update updates window, just call it at the end
    pub fn update(&mut self) {
        let size = self.size();
        if size != self.canvas.size() {
            self.canvas.resize(TextureSize::new(size.0 as i32, size.1 as i32))
        }
        self.canvas.draw(&mut self.window, &Mat::IM, &WHITE);
        self.gl_swap_window();
    }

    /// get_viewport_rect returns rectangle that whole screen fits in and that is even if you
    /// rotate camera. Useful when you don't want to draw sprites that are foo screen
    pub fn get_viewport_rect(&self) -> Rect {
        let (w, h) = self.size();
        let w = w as i32 /2;
        let h = h as i32 /2;

        let mut corners: [Vect; 4] = [
            vect!(-w, -h),
            vect!(w, -h),
            vect!(w, h),
            vect!(-w, h),
        ];

        for i in 0..corners.len() {
            corners[i] = self.canvas.camera.unprj(corners[i]);
        }

        Rect::bounds_for(&corners)
    }

    #[inline]
    pub fn clear(&self) {
        self.canvas.clear();
    }
}

impl Target for video::Window {
    fn append(&mut self, data: &[f32], pattern: &[u32], _: u32, program: Option<&Program>, texture: Option<&Texture>, buffer: Option<&Buffer>) {
        let (w, h) = self.size();

        unsafe {
            gl::Viewport(0, 0, w as i32, h as i32);
        }

        match program {
            Some(p) => {
                p.bind();
                p.set_camera(Mat::IM);
                p.set_view_size(vect!(w, h));
            }
            None => panic!("program mustn't be none when drawing to window"),
        }

        if let Some(t) = texture {
            t.bind()
        }

        let buffer = match buffer {
            Some(b) => b,
            None => panic!("buffer mustn't be none when drawing to window"),
        };

        buffer.set_vertices_and_indices(data, pattern);
        buffer.bind();
        buffer.draw(pattern.len());
    }
}