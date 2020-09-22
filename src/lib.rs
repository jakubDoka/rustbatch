//! This crate provides some performant tools for building huge 2D games that deals with huge amount of entities.
//! So far crate contains:
//! - OpenGl abstraction based around batching.
//! - Collision scanner that provides fast collision detection for couple thousand entities
//! - Multithreaded pathfinder that has no problems ewen with 1000 X 1000 tile-maps
//! - Math module that contains structs like Matrix, Vector or Rectangle
//! - Custom per-batch vertex and fragment shader also supported
//! - Sprite packing so you can take advantage of batching as match as you can
//! # Example
//! ```
//!extern crate rustbatch;
//!
//!use rustbatch::{sdl2, image, gl};
//!use rustbatch::debug::FPS;
//!use rustbatch::{Window, Texture, Sprite, Batch};
//!use rustbatch::{IM, WHITE};
//!
//!
//!fn main() {
//!    // creating window to draw to and event pump to read input
//!    // just ignore gl here it cannot be dropped otherwise rendering will not work so just leave it be
//!    let (mut window, mut event_pump, gl) = Window::new(|sys| {
//!        sys.window("rusty batch", 400, 400)
//!            .opengl()
//!            .resizable()
//!            .build()
//!            .unwrap()
//!    });
//!
//!    window.set_background_color(&[0.5f32, 0.5f32, 0.5f32, 1f32]); //gray background
//!
//!    // use of image crate to load image
//!    let img = image::open("C:/Users/jakub/Documents/programming/rust/src/ggl/src/bullets.png").unwrap();
//!
//!    // This is wrapped opengl texture object
//!    let texture = Texture::from_img(
//!        &img,
//!        gl::NEAREST, // So the pixels are drawn as they are
//!        gl::RGBA // Color structure, you would use gl::RGB if your texture does not have alpha channel
//!    );
//!
//!    // Creating sprite. Notice that sprite is just collection of points and it cannot be directly
//!    // drawn to window
//!    let mut sprite = Sprite::new(texture.frame());
//!
//!    // On the other hand batch owns texture witch can be drawn to window
//!    let mut batch = Batch::new(texture);
//!
//!    // this is just a little helper
//!    let mut fps = FPS::new(1f32);
//!
//!    'main: loop {
//!        //polling events
//!        for event in event_pump.poll_iter() {
//!            match event {
//!                // break loop if X button on window is pressed
//!                sdl2::event::Event::Quit { .. } => break 'main,
//!                _ => {}
//!            }
//!        }
//!
//!        // i hope you know how to get delta on your own but fps returns is as bonus if you supply
//!        // 0f32 as delta
//!        let _delta = fps.increase(0f32);
//!
//!        // clearing window
//!        window.clear();
//!
//!        // drawing sprite to batch
//!        // IM is matrix transform. more about matrix in mat module docs
//!        // texture color is multiplied by inputted color
//!        sprite.draw(&mut batch, &IM, &WHITE);
//!
//!        // drawing batch to window
//!        window.draw(&batch);
//!
//!        // Don't forget to clear batch if you not planning to use it as canvas
//!        // after all drawing sprites to batch takes some time
//!        batch.clear();
//!
//!        // finishing with window update so you can se it changing
//!        window.update();
//!    }
//!}
//! ```
pub mod images;
pub mod entity;
pub mod render;
pub mod debug;
pub mod math;

pub use sdl2;
pub use image;
pub use gl;

pub use debug::FPS;
pub use render::{window::Window, texture::Texture, sprite::Sprite, batch::Batch};
pub use math::{mat::IM, rgba::WHITE};



