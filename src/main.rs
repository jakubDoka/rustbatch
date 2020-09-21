#![allow(dead_code)]
mod images;
mod entity;
mod render;
mod debug;

extern crate gl;
extern crate glm;
extern crate sdl2;

use rand::Rng;

use math::vect::Vect;

use crate::math::{vect, mat, rgba};
use render::window::Window;
use render::texture;
use crate::render::batch::Batch;
use crate::debug::FPS;
use crate::entity::scanner::Scanner;
use crate::math::rect::Rect;
use crate::entity::id_generator::IDGenerator;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Instant;
use crate::entity::pathfinder;
use std::thread;
use std::sync::mpsc::channel;
use crate::render::sprite::Sprite;

mod math;

struct Body {
    id: u64,
    pos: Vect,
    prev_pos: Vect,
    vel: Vect,
}

impl Body {
    pub fn step(&mut self, delta: f32) {
        self.prev_pos = self.pos;
        self.pos += self.vel * delta;
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("Game", 500, 500)
        .opengl()
        .resizable()
        .build()
        .unwrap();



    let gl_context = window.gl_create_context().unwrap();


    Window::init(&window, &gl_context, &video_subsystem);

    let mut window = Window::new(window);

    let img = image::open("C:/Users/jakub/Documents/programming/rust/src/rustbatch/src/icon.png").unwrap();

    let sheet = images::Sheet::new(&[img.clone(), img]);

    let texture = texture::Texture::from_img(
        sheet.pic,
        gl::NEAREST,
        gl::RGBA
    );

    //let mut sprite = Sprite::new(texture.frame());

    let mut sprite = Sprite::new(texture.frame());

    let mut batch = Batch::new(texture);

    let mut fps = FPS::new(1f32);
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        let delta = fps.increase(0f32);

        window.clear();


        window.draw(&batch);
        batch.clear();

        window.update();
    }
}