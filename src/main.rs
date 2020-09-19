

#![allow(dead_code)]
mod entity;
mod render;


extern crate gl;
extern crate glm;
extern crate sdl2;

use std::f32::consts::PI;
use std::ffi::CString;
use std::sync::Arc;

use image::GenericImageView;
use rand::Rng;
use sdl2::VideoSubsystem;

use math::vect::Vect;

use crate::batch::Batch;
use crate::buffer::Buffer;
use crate::math::{mat, rgba, vect};
use render::sprite::Sprite;
use render::window::Window;
use render::texture;

mod math;
mod batch;
mod buffer;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("Game", 500, 500)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();


    Window::init(&window, &video_subsystem);

    let mut window = Window::new(window);

    let texture = texture::Texture::new(
        "C:/Users/jakub/Documents/programming/rust/src/rustbatch/src/icon.png",
        gl::NEAREST,
        gl::RGBA
    ).unwrap();

    let mut sprite = Sprite::new(texture.frame());

    let mut batch = Batch::new(texture);

    let mut rng = rand::thread_rng();

    let mut poss = [mat::IM; 10000];

    for i in 0..10000 {
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();
        let r: f32 = rng.gen();
        poss[i] = mat::IM.rotated(vect::ZERO, r * 10f32).moved(Vect::new(500f32*x, 500f32*y));
    }


    //window.set_camera(mat::IM.scaled(vect::ZERO, 0.5f32));

    let mut now = std::time::Instant::now();
    let mut t = 0f32;
    let mut f = 0i32;

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        f += 1;
        let delta = std::time::Instant::elapsed(&now).as_secs_f32();
        t += delta;
        now = std::time::Instant::now();
        if t > 1f32 {
            t = 0f32;
            println!("{}", f);
            f = 0;
        }

        for i in 0..10000 {
            sprite.draw(&mut batch, &poss[i], &rgba::WHITE);
        }

        window.clear();

        window.draw(&batch);
        batch.clear();

        window.update();
    }
}