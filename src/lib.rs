//! In order to get the crate running, you have to copy `msvc` folder and `build.rs` from crates
//! [repository](https://github.com/jakubDoka/rustbatch)
//! and place it into root of your project.
//! # RustBatch
//! This crate provides some performant tools for building big 2D games that deals with huge amount of entities.
//! So far crate contains:
//! - OpenGl abstraction based around batching.
//! - Collision scanner that provides fast collision detection for couple thousand entities
//! - Multithreaded pathfinder that has no problems ewen with 1000 X 1000 tile-maps
//! - Math module that contains structs like Matrix, Vector or Rectangle
//! - Custom per-batch vertex and fragment shader also supported
//! - Sprite packing so you can take advantage of batching as match as you can
//! # Warming
//! Important thing to note is that some functions from render module are calling functions and
//! using enums from gl crate. Functions has to be loaded first witch you achieve by creating
//! window.
//! # Example
//! ```
//!extern crate rustbatch;
//!
//!use rustbatch::{sdl2, image, gl};
//!use rustbatch::debug::FPS;
//!use rustbatch::{Window, Texture, Sprite, Batch};
//!use rustbatch::{Mat, WHITE, Vect};
//!
//!
//!fn main() {
//!    // creating window to draw to and event pump to read input. Ignore
//!    // gl var, it cannot be dropped otherwise rendering will not work so just leave it be
//!
//!    use rustbatch::render::texture::TextureConfig;
//! let (mut window, mut event_pump, _gl, _sdl, _video_subsystem) = Window::new(|sys| {
//!        sys.window("rusty batch", 400, 400)
//!            .opengl()
//!            .resizable()
//!            .build()
//!            .unwrap()
//!    });
//!
//!    window.canvas.set_background_color(&[0.5f32, 0.5f32, 0.5f32, 1f32]); //gray background
//!
//!    // This is wrapped opengl texture object
//!    let texture = Texture::new(
//!        "C:/Users/jakub/Documents/programming/rust/src/rustbatch/assets/logo.png",
//!       TextureConfig::DEFAULT,
//!    ).unwrap();
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
//!        //
//!        window.canvas.clear();
//!
//!        // drawing sprite to batch
//!        // texture color is multiplied by inputted color
//!        sprite.draw(&mut batch, Vect::ZERO, Vect::mirror(1f32), 0f32, &WHITE);
//!
//!        // drawing batch to window
//!        batch.draw(&mut window.canvas);
//!
//!        // Don't forget to clear batch if you not planning to use it as canvas,
//!        // after all drawing sprites to batch takes some time
//!        batch.clear();
//!
//!        // finishing with window update so you can se it changing
//!        window.update();
//!    }
//!}
//! ```


#[macro_export]
macro_rules! vect {
        ($x: expr, $y: expr) => {
            Vect { x: $x as f32, y: $y as f32}
        }
    }

#[macro_export]
macro_rules! rect {
        ($x: expr, $y: expr, $x1: expr, $y1: expr) => {
            Rect{
                min: Vect {
                    x: $x as f32,
                    y: $y as f32,
                },
                max: Vect {
                    x: $x1 as f32,
                    y: $y1 as f32,
                },
            }
        }
    }

#[macro_export]
macro_rules! curve {
    ($ax: expr, $ay: expr; $ahx: expr, $ahy: expr; $bx: expr, $by: expr; $bhx: expr, $bhy: expr) => {
        Curve {
            a: Vect {x: $ax as f32, y: $ay as f32},
            a_handle: Vect {x: $ahx as f32, y: $ahy as f32},
            b: Vect {x: $bx as f32, y: $by as f32},
            b_handle: Vect {x: $bhx as f32, y: $bhy as f32},
            placeholder: false,
        }
    }
}

pub mod images;
pub mod render;
pub mod debug;
pub mod math;
pub mod entity;

pub use sdl2;
pub use image;
pub use gl;
pub use rand;

pub use debug::FPS;
pub use render::{window::Window, texture::Texture, sprite::Sprite, batch::Batch};
pub use math::{mat::Mat, rgba::WHITE, vect::Vect};




