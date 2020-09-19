#![allow(dead_code)]
mod entity;
mod render;
mod debug;

extern crate gl;
extern crate glm;
extern crate sdl2;

use rand::Rng;

use math::vect::Vect;

use crate::math::vect;
use render::window::Window;
use render::texture;
use crate::render::batch::Batch;
use crate::debug::FPS;
use crate::entity::scanner::Scanner;
use crate::math::rect::Rect;
use crate::entity::id_generator::IDGenerator;

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
    let texture = texture::Texture::new(
        "C:/Users/jakub/Documents/programming/rust/src/rustbatch/src/icon.png",
        gl::NEAREST,
        gl::RGBA
    ).unwrap();

    //let mut sprite = Sprite::new(texture.frame());

    let mut batch = Batch::new(texture);

    let mut bodies = Vec::new();

    let mut map = Scanner::new(1000, 1000, Vect::i32(50, 50));

    let mut rng = rand::thread_rng();
    let mut gen = IDGenerator::new();

    for _ in 0..10000 {
        let b = Body{
            pos: Vect::new(rng.gen::<f32>() * 2000f32, rng.gen::<f32>() * 2000f32),
            vel: Vect::new(rng.gen::<f32>() * 300f32, rng.gen::<f32>() * 300f32),
            id: gen.gen(),
            prev_pos: vect::ZERO,
        };

        map.insert(&b.pos, b.id);

        bodies.push(b);
    }

    let mut collector = Vec::new();

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

        for b in bodies.iter_mut() {
            b.step(delta);
            map.update(&b.prev_pos, &b.pos, b.id);
            map.query(&Rect::centered(b.pos, 50f32, 50f32), &mut collector);
            collector.clear()
        }



        window.clear();

        window.draw(&batch);
        batch.clear();

        window.update();
    }
}