use crate::math::curve::Curve;
use crate::{ Vect, Sprite};
use crate::render::batch::{Target, VertexData};
use crate::math::rgba::{RGBA, Graph};
use rand::Rng;
use rand::rngs::ThreadRng;

#[inline]
pub fn in_range(random: &mut ThreadRng , min: f32, max:f32) -> f32 {
    random.gen::<f32>() * (max - min) + min
}

pub trait Particle {
    fn draw(&mut self, target: &mut VertexData, position: Vect, rotation: f32, scale: f32, color: &RGBA);
}

pub trait Generator {
    fn gen(&mut self) -> Vect;
}

#[derive(Copy, Clone)]
pub struct Property {
    pub curve: Curve,
    pub value: f32,
}

impl Property {
    #[inline]
    fn get(&self, t: f32) -> f32 {
        self.curve.get_point(t).y * self.value
    }
}

#[derive(Copy, Clone)]
pub struct RandomizedProperty {
    value: f32,
    offset: f32,
}

impl RandomizedProperty {
    #[inline]
    pub fn new(value: f32, offset: f32) -> Self {
        Self { value, offset }
    }

    #[inline]
    pub fn no_random(value: f32) -> Self {
        Self{ value, offset: 0.0 }
    }

    #[inline]
    pub fn get(&self, randomizer: &mut ThreadRng) -> f32 {
        self.value + randomizer.gen::<f32>() * 2.0 * self.offset - self.offset
    }
}

struct Object {
    pos: Vect,
    vel: Vect,
    origin: Vect,

    twerk: f32,
    rot: f32,
    live_time: f32,
    progress: f32,

}

#[derive(Copy, Clone)]
pub struct Initial {
    pub velocity: RandomizedProperty,
    pub rotation: RandomizedProperty,
    pub twerk: RandomizedProperty,
    pub live_time: RandomizedProperty,
}

#[derive(Copy, Clone)]
pub struct Dynamic {
    pub acceleration: Property,
    pub twerk_acceleration: Property,
    pub scale: Property,
}



#[derive(Clone)]
pub struct ParticleConfig {
    pub rotation_relative_to_spawn_direction: bool,
    pub inverted: bool,
    pub gravity: Vect,
    pub color: Graph,

    pub spread: f32,
    pub friction: f32,
    pub origin_attraction: f32,


    pub initial: Initial,
    pub dynamic: Dynamic,

}

pub struct Gen {}

impl Generator for Gen {
    fn gen(&mut self) -> Vect {
        Vect::ZERO
    }
}

struct Inner {
    random: ThreadRng,
    config: ParticleConfig,
    objects: Vec<Object>,
    generator: Box<dyn Generator>,
    pos: Vect,
    dir: Vect,
}

impl Inner {
    pub fn new_obj(&mut self) -> Object {
        let mut obj = Object{
            pos: self.pos + self.generator.gen(),
            vel: Vect::rad(self.dir.ang() + in_range(&mut self.random,-self.config.spread, self.config.spread),
                           self.config.initial.velocity.get(&mut self.random)),
            origin: self.pos,
            twerk: self.config.initial.twerk.get(&mut self.random),
            rot: self.config.initial.rotation.get(&mut self.random),
            live_time: self.config.initial.live_time.get(&mut self.random),
            progress: 0.0
        };

        if self.config.rotation_relative_to_spawn_direction {
           obj.rot += obj.vel.ang();
        }

        obj
    }
}
/// ParticleSystem is particle system. You can use custom shapes and position generator. other way to customize
/// particles it though config.
///
/// # example
/// ```
/// use rustbatch::{Window, Batch, Vect, FPS};
/// use rustbatch::render::particle::system::{ParticleSystem, Initial, RandomizedProperty, Dynamic, Property};
/// use rustbatch::render::particle::system;
/// use rustbatch::math::rgba::{Graph, GraphPoint};
/// use std::f32::consts::PI;
/// use rustbatch::math::curve::Curve;
/// use rustbatch::render::particle::shapes::{SymmetricShape, Triangle};
/// use rustbatch::vect;
///
/// let (mut window, mut pump, _f, _g, _r) = Window::new(|sys| sys.window("segmentation", 400, 400).opengl().build().unwrap());
///     let mut batch = Batch::no_texture();
///     let mut sys = ParticleSystem::no_texture(system::ParticleConfig {
///         rotation_relative_to_spawn_direction: true,
///         inverted: false,
///
///         gravity: Vect::DOWN * 2.0,
///         color: Graph::new(vec![GraphPoint::new(0.0, [0.0, 0.0, 1.0, 0.5]), GraphPoint::new(0.5, [1.0, 1.0, 0.0, 1.0]), GraphPoint::new(1.0, [0.0, 1.0, 0.0, 0.0])]),
///         spread: PI,
///         friction: 2.0,
///
///         origin_attraction: 10.0,
///         initial: Initial {
///             velocity: RandomizedProperty::new(300.0, 150.0),
///             rotation: RandomizedProperty::no_random(0.0),
///             twerk: RandomizedProperty::no_random(10.0),
///             live_time: RandomizedProperty::new(1.0, 0.5),
///         },
///         dynamic: Dynamic {
///             acceleration: Property { curve: Curve::NONE, value: 0.0 },
///             twerk_acceleration: Property { curve: Curve::NONE, value: 0.0 },
///             scale: Property { curve: Curve::NONE, value: 20.0 }
///         }
///     }, Box::new(Triangle::new(&[Vect::unit(0.0), Vect::unit(2.0 * PI/3.0), Vect::unit(4.0 * PI/3.0)])));
///
///     let mut fps = FPS::new(1f32);
///     'main: loop {
///         for event in pump.poll_iter() {
///             match event {
///                 /// break loop if X button on window is pressed
///                 sdl2::event::Event::Quit { .. } => break 'main,
///                 _ => {}
///             }
///         }
///
///         let delta = fps.increase(0f32);
///
///
///         window.clear();
///
///         sys.update(delta);
///
///         sys.spawn(10);
///
///         sys.draw(&mut batch);
///
///         sys.clear();
///
///         batch.draw(&mut window.canvas);
///
///         batch.clear();
///
///         window.update();
///
///     }
/// ```
pub struct ParticleSystem {
    shape: Box<dyn Particle>,
    progress: f32,
    vertex_data: VertexData,
    objects: Vec<Object>,
    inner: Inner,
}

impl ParticleSystem {
    pub fn new(config: ParticleConfig, sprite: Sprite) -> Self {
        Self::customized(VertexData::new(), config, Box::new(sprite), Box::new(Gen {}))
    }

    pub fn no_texture(config: ParticleConfig, shape: Box<dyn Particle>) -> Self {
        Self::customized(VertexData::no_texture(), config, shape, Box::new(Gen {}))
    }

    pub fn customized(vertex_data: VertexData, config: ParticleConfig, shape: Box<dyn Particle>, generator: Box<dyn Generator>) -> Self {
        Self {
            inner: Inner {
                random: rand::thread_rng(),
                config,
                generator,
                pos: Vect::ZERO,
                objects: vec![],
                dir: Vect::ZERO,
            },
            progress: 0.0,
            vertex_data,
            shape,
            objects: vec![],
        }
    }



    pub fn spawn(&mut self, count: usize) {
        self.objects.reserve(count);

        if self.inner.config.inverted {
            for _ in 0..count {
                self.objects.insert(0, self.inner.new_obj())
            }
        } else {
            for _ in 0..count {
                self.objects.push(self.inner.new_obj())
            }
        }


    }

    pub fn update(&mut self, delta: f32) {
        for mut obj in self.objects.drain(..) {

            obj.progress += delta;
            self.progress = obj.progress / obj.live_time;
            if self.progress >= 1.0 {
                continue;
            }

            self.shape.draw(&mut self.vertex_data, obj.pos, obj.rot, self.inner.config.dynamic.scale.get(self.progress), &self.inner.config.color.get_color(self.progress));
            obj.vel += (obj.vel.norm() * (self.inner.config.dynamic.acceleration.get(self.progress)) + self.inner.config.gravity - obj.vel * self.inner.config.friction + (obj.origin - obj.pos).norm() * self.inner.config.origin_attraction) * delta;
            obj.twerk += (self.inner.config.dynamic.twerk_acceleration.get(self.progress) - obj.twerk * self.inner.config.friction) * delta;
            obj.pos += obj.vel * delta;
            obj.rot += obj.twerk * delta;

            self.inner.objects.push(obj);
        }

        std::mem::swap(&mut self.inner.objects, &mut self.objects);
    }

    #[inline]
    pub fn draw<T: Target>(&self, target: &mut T) {
        target.append(&self.vertex_data.vertices, &self.vertex_data.indices, self.vertex_data.vertex_size, None, None, None)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.vertex_data.clear()
    }
}

