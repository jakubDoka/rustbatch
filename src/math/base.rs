use crate::Vect;
use std::mem;

#[derive(Copy, Clone, Debug)]
pub struct  Base {
    pub a: Vect,
    pub b: Vect,
    l: f32,
}

impl Base {
    pub const ZERO: Self = Self{a: Vect::ZERO, b: Vect::ZERO, l: 0f32};

    #[inline]
    pub fn new(a: Vect, b: Vect) -> Self {
        Self{ a, b, l: (a - b).len() }
    }

    #[inline]
    pub fn ang(&self) -> f32 {
        (self.a - self.b).ang()
    }

    #[inline]
    pub fn center(&self) -> Vect {
        self.a + (self.b - self.a)/2f32
    }

    pub fn mv(&mut self, delta: Vect) {
        self.a += delta;
        self.b += delta;
    }

    #[inline]
    pub fn rotate_around(&mut self, delta: f32, point: Vect) {
        self.mv(point.inverted());
        self.a = self.a.rot(delta);
        self.b = self.b.rot(delta);
        self.mv(point);
    }

    #[inline]
    pub fn rotate(&mut self, delta: f32) {
        self.rotate_around(delta, self.center());
    }

    #[inline]
    pub fn dir(&self) -> Vect {
        self.a - self.b
    }

    #[inline]
    pub fn set_pos(&mut self, pos: Vect) {
        let d = self.dir()/2f32;
        self.a = pos + d;
        self.b = pos - d;
    }

    #[inline]
    pub fn set_rot(&mut self, rot: f32) {
        let d = Vect::rad(rot ,self.l/2f32);
        let c = self.center();
        self.a = d + c;
        self.b = d.inverted() + c;
    }

    #[inline]
    pub fn pull(&mut self, mut force: Vect) -> Vect {
        let l = force.len();
        let mut extra = Vect::ZERO;

        if l > self.l {
            extra = force.norm() * self.l;
            mem::swap(&mut self.a, &mut self.b);
            self.a = self.b + extra;
            force -= extra;
        }

        let dir = self.dir();

        let projected = Vect::rad(dir.ang_to(force), l);
        let back_move = self.l - (self.l*self.l-projected.y*projected.y).sqrt() + projected.x;
        let back_force = dir.norm() * back_move;

        self.a += force;
        self.b += back_force;

        back_force + extra
    }
}