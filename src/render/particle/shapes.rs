use crate::{Mat, Vect};
use std::f32::consts::PI;
use crate::math::rgba::RGBA;
use crate::render::batch::{Target, VertexData};
use crate::render::particle::system::Particle;

/// SymmetricShape is used for drawing shapes like triangles rectangles all the way to circles,
/// it depends on how many edges you specify. Note that this is not at all more effective then
/// drawing sprite with that shape and also you have to use no_texture batch.
/// It gives you option to draw with whatever color you want.
/// You can even draw shape with lets say white edges and transparent center.
/// # example
/// ```
/// use rustbatch::{Mat, Vect, WHITE, Batch, FPS, Window};
/// use rustbatch::math::rgba::BLACK;
/// use rustbatch::render::particle::shapes::SymmetricShape;
/// fn main() {
///
///     let (mut window, mut event_pump, _gl, _s, _e) = Window::new(|sys| {
///         sys.window("heureka", 1000, 600)
///             .opengl()
///             .resizable()
///             .build()
///             .unwrap()
///     });
///
///     window.canvas.set_background_color(&[0.5f32, 0.5f32, 0.5f32, 1f32]);
///
///     //creating shape with 10 edges
///     let mut shape = SymmetricShape::new(10, 0f32);
///
///     //this is important, you have to use no texture batch to use shapes in it
///     let mut batch = Batch::no_texture();
///
///     let mut fps = FPS::new(1f32);
///
///     'main: loop {
///         //polling events
///         for event in event_pump.poll_iter() {
///             match event {
///                 sdl2::event::Event::Quit { .. } => break 'main,
///                 _ => {}
///             }
///         }
///
///         let delta = fps.increase(0f32);
///
///         window.canvas.clear();
///
///         shape.draw(&mut batch, &Mat::IM.scaled(Vect::ZERO, 100f32), &WHITE, &BLACK);
///
///         batch.draw(&mut window.canvas);
///
///
///         batch.clear();
///
///         window.update();
///     }
/// }
/// ```
pub struct SymmetricShape {
    points: Vec<Vect>,
    buffer: Vec<f32>,
    indices: Vec<u32>,
}

impl Clone for SymmetricShape {
    #[inline]
    fn clone(&self) -> Self {
        Self{
            points: self.points.clone(),
            buffer: self.buffer.clone(),
            indices: self.indices.clone(),
        }
    }
}

impl SymmetricShape {
    /// new is shape constructor
    #[inline]
    pub fn new(edges: usize, rotation: f32) -> SymmetricShape {
        let mut points = Vec::with_capacity(edges);
        for i in 0..edges{
            points.push(Vect::unit(PI*2f32/edges as f32 * i as f32 + rotation));
        }

        let mut indices = Vec::with_capacity(edges * 3);
        for i in 0..(edges as u32 - 1) {
            indices.extend(&[0, i + 1, i + 2])
        }
        indices.extend(&[0, edges as u32, 1]);

        SymmetricShape{
            points,
            buffer: Vec::with_capacity(edges * 6 + 6),
            indices
        }
    }

    /// draw draws shape to batch, batch has to be `no_texture` batch otherwise this triggers panic.
    /// See batch functions to how to make no_texture batch.
    #[inline]
    pub fn draw<T: Target>(&mut self, target: &mut T, transform: &Mat, inner_color: &RGBA, outer_color: &RGBA) {
        self.buffer.clear();
        self.buffer.extend(&[transform.c.x, transform.c.y]);
        self.buffer.extend(inner_color);
        for p in self.points.iter() {
            let prj = transform.prj(*p);
            self.buffer.extend(&[prj.x, prj.y]);
            self.buffer.extend(outer_color);
        }

        target.append(&self.buffer, &self.indices, 6, None, None, None);
    }
}

impl Particle for SymmetricShape {
    fn draw(&mut self, target: &mut VertexData, position: Vect, rotation: f32, scale: f32, color: &RGBA) {
        self.draw(target, &Mat::new(position,Vect::new(scale, scale), rotation), color, color);
    }
}

pub struct Triangle {
    points: [Vect; 3],
    buff: [f32; 18],
    pos: Vect,
}

impl Triangle {
    pub const PATTERN: [u32; 3] = [0, 1, 2];

    pub fn new(points: &[Vect; 3]) -> Self {
        Self{ points: points.clone(), buff: [0f32; 18], pos: Vect::ZERO }
    }

    pub fn draw<T: Target>(&mut self, target: &mut T, transform: &Mat, color: &RGBA) {
        for (mut i, pos) in self.points.iter().enumerate() {
            self.pos = transform.prj(*pos);
            i *= 6;
            self.buff[i+0] = self.pos.x;
            self.buff[i+1] = self.pos.y;
            self.buff[i+2..i+6].copy_from_slice(color);
        }

        target.append(&self.buff, &Self::PATTERN, 6, None, None, None);
    }
}

impl Particle for Triangle {
    fn draw(&mut self, target: &mut VertexData, position: Vect, rotation: f32, scale: f32, color: &RGBA) {
        self.draw(target, &Mat::new(position, Vect::new(scale, scale), rotation), color);
    }
}