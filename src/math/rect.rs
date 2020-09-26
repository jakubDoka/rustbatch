use crate::math::vect;
use crate::math::vect::Vect;
use std::mem;


/// Rect is Rectangular shape or AABB for detecting collisions in 2D space
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub min: Vect,
    pub max: Vect,
}

pub enum Sides {
    Right, Left, Top, Bottom, In
}

impl Rect {
    pub const ZERO: Rect = Rect{min: Vect::ZERO, max: Vect::ZERO};
    pub const INVERTED_MAX_RECT: Rect = Rect{min: Vect::MAX, max: Vect::MIN};
    pub const MAX_RECT: Rect = Rect{min: Vect::MIN, max: Vect::MAX};

    /// new is rect constructor
    #[inline]
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Rect {
        Rect {
            min: Vect { x: x0, y: y0 },
            max: Vect { x: x1, y: y1 },
        }
    }

    /// wn lets you specify width and height of rectangle rather then two points
    #[inline]
    pub fn wh(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            min: Vect { x, y },
            max: Vect { x: x + w, y: y + h },
        }
    }

    /// centered returns rect that has center in c
    #[inline]
    pub fn centered(c: Vect, mut w: f32, mut h: f32) -> Rect {
        w /= 2f32;
        h /= 2f32;
        Rect {
            min: Vect { x: c.x - w, y: c.y - h },
            max: Vect { x: c.x + w, y: c.y + h },
        }
    }

    /// cube is similar to centered but you specify just radius of cube
    /// with means that if rad is 5 then width and height are 10
    #[inline]
    pub fn cube(c: Vect, rad: f32) -> Rect {
        Rect {
            min: Vect { x: c.x - rad, y: c.y - rad },
            max: Vect { x: c.x + rad, y: c.y + rad },
        }
    }

    /// from_vec turns vector to rectangle
    #[inline]
    pub fn from_vec(v: Vect) -> Rect {
        Rect {
            min: Vect::ZERO,
            max: v,
        }
    }

    /// bounds for returns rectangle where all points from slice fits in
    #[inline]
    pub fn bounds_for(points: &[Vect]) -> Rect {
        if points.is_empty() {
            return Self::ZERO;
        }

        let mut bounds = Self::INVERTED_MAX_RECT;

        for p in points {
            if p.x > bounds.max.x {
                bounds.max.x = p.x;
            }
            if p.x < bounds.min.x {
                bounds.min.x = p.x;
            }
            if p.y > bounds.max.y {
                bounds.max.y = p.y;
            }
            if p.y < bounds.min.y {
                bounds.min.y = p.y;
            }
        }

        bounds
    }

    /// verts returns corner points of rectangle
    #[inline]
    pub fn verts(&self) -> [Vect; 4] {
        [
            self.min,
            Vect { x: self.min.x, y: self.max.y },
            self.max,
            Vect { x: self.max.x, y: self.min.y }
        ]
    }

    /// union returns smallest rectangle in witch both self and o fits in
    #[inline]
    pub fn union(&self, o: &Rect) -> Rect {
        Rect {
            min: Vect { x: self.min.x.min(o.min.x), y: self.min.y.min(o.min.y) },
            max: Vect { x: self.max.x.max(o.max.x), y: self.max.y.max(o.max.y) },
        }
    }

    /// center returns center of rectangle
    #[inline]
    pub fn center(&self) -> Vect {
        self.min + (self.max - self.min) / 2f32
    }

    /// loc_verts returns corners of rectangle relative to its center
    #[inline]
    pub fn loc_verts(&self) -> [Vect; 4] {
        let c = self.center();
        let mut verts = self.verts();
        for i in 0..4 {
            verts[i] -= c;
        }
        verts
    }

    /// to_local returns rect centered around coordinate origin (0, 0)
    #[inline]
    pub fn to_local(&self) -> Rect {
        Self::centered(Vect::ZERO, self.width(), self.height())
    }

    /// width returns rectangles width
    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// height returns rectangles height
    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// intersects returns whether rectangle intersects another rectangle
    #[inline]
    pub fn intersects(&self, o: &Rect) -> bool {
        !(self.max.x < o.min.x || self.max.y < o.min.y || o.max.x < self.min.x || o.max.y < self.min.y)
    }

    /// respective returns where the point is respective to rectangle
    pub fn respective(&self, pos: Vect) -> Sides {
        if pos.x < self.min.x {
            return Sides::Left;
        }
        if pos.x > self.max.x {
            return Sides::Right;
        }
        if pos.y < self.min.y {
            return Sides::Bottom;
        }
        if pos.y > self.max.y {
            return Sides::Top;
        }
        Sides::In
    }

    /// normalized normalizes rectangle so the min is lower them max
    #[inline]
    pub fn normalized(mut self) -> Rect {
        if self.min.x > self.max.x {
            mem::swap(&mut self.min.x , &mut self.max.x)
        }

        if self.min.y > self.max.y {
            mem::swap(&mut self.min.y , &mut self.max.y)
        }

        self
    }

    /// contains returns whether rect contains the points
    #[inline]
    pub fn contains(&self, pos: Vect) -> bool {
        self.max.x > pos.x && self.min.x < pos.x && self.max.y > pos.y && self.min.y < pos.y
    }

    /// fits_in returns whether self fits in o
    #[inline]
    pub fn fits_in(&self, o: &Rect) -> bool {
        self.max.x <= o.max.x && self.max.y <= o.max.y && o.min.x <= self.min.x && o.min.y <= self.min.y
    }

    /// radius returns distance from rect center to max
    #[inline]
    pub fn radius(&self) -> f32 {
        (self.max - self.max).len() / 2f32
    }

    /// moved returns rectangle moved by delta
    #[inline]
    pub fn moved(&self, delta: Vect) -> Self {
        Rect {
            min: self.min + delta,
            max: self.max + delta,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::rect::Rect;

    #[test]
    fn intersects_test() {
        let base = Rect::new(0f32, 0f32, 10f32, 10f32);
        assert!(base.intersects(&base));
        assert!(base.intersects(&Rect::new(10f32, 10f32, 100f32, 100f32)));
        assert!(!base.intersects(&Rect::new(100f32, 100f32, 1000f32, 1000f32)));
    }
}