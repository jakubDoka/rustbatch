mod buffer;
mod mat;
mod vect;
mod program;
mod texture;

extern crate gl;
extern crate sdl2;
extern crate glm;

use std::sync::Arc;
use std::ffi::CString;
use image::GenericImageView;
use crate::vect::Vect;
use crate::buffer::Buffer;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 500, 500)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let program = program::Program::default();

    let texture = texture::Texture::new(
        "C:/Users/jakub/Documents/programming/rust/src/rustbatch/src/icon.png",
        gl::NEAREST,
        gl::RGBA
    ).unwrap();

    let size = texture.size();

    let vertices: Vec<f32> = vec![
        //pos         tex               col
        0.0, 0.0,   0.0, 0.0,    1.0, 1.0, 0.0, 0.0,
        250.0, 0.0,   size.x, 0.0,    0.0, 1.0, 1.0, 1.0,
        250.0, 250.0,   size.x, size.y,    1.0, 0.0, 1.0, 1.0,
        0.0, 250.0,   0.0, size.y,    0.0, 1.0, 0.0, 1.0,
    ];

    let indices: Vec<u32> = vec![
        0,1,3,1,2,3
    ];


    // set up vertex array object

    let buffer = Buffer::new();

    buffer.set_vertices_and_indices(vertices, indices);

    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, 500, 500);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }



    program.set_transform_matrix(mat::IM.moved(Vect::new(-1f32, -1f32)));
    program.set_view_size(Vect::new(250f32, 250f32));
    program.set_texture_size(texture.size());
    program.set_camera(mat::IM.moved(Vect::new(1f32, 1f32)));
    // main loop
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangle

        program.set_used();

        texture.set_used();
        buffer.set_used();
        buffer.draw();

        window.gl_swap_window();


    }
}