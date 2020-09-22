use crate::math::vect;
use crate::math::vect::Vect;

pub static ZERO: Rect = Rect{min: vect::ZERO, max: vect::ZERO};

#[derive(Copy, Clone)]
pub struct Rect {
    pub min: Vect,
    pub max: Vect,
}

impl Rect {
    #[inline]
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Rect {
        Rect {
            min: Vect { x: x0, y: y0 },
            max: Vect { x: x1, y: y1 },
        }
    }
    #[inline]
    pub fn wh(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            min: Vect { x, y },
            max: Vect { x: x + w, y: y + h },
        }
    }
    #[inline]
    pub fn centered(c: Vect, w: f32, h: f32) -> Rect {
        Rect {
            min: Vect { x: c.x - w / 2f32, y: c.y + h / 2f32 },
            max: Vect { x: c.x + w / 2f32, y: c.y + h / 2f32 },
        }
    }
    #[inline]
    pub fn from_vec(v: Vect) -> Rect {
        Rect {
            min: vect::ZERO,
            max: v,
        }
    }
    #[inline]
    pub fn verts(&self) -> [Vect; 4] {
        [
            self.min,
            Vect { x: self.min.x, y: self.max.y },
            self.max,
            Vect { x: self.max.x, y: self.min.y }
        ]
    }
    #[inline]
    pub fn center(&self) -> Vect {
        self.min + (self.max - self.min) / 2f32
    }
    #[inline]
    pub fn loc_verts(&self) -> [Vect; 4] {
        let c = self.center();
        let mut verts = self.verts();
        for i in 0..4 {
            verts[i] -= c;
        }
        verts
    }
    #[inline]
    pub fn to_local(&self) -> Rect {
        Self::centered(vect::ZERO, self.width(), self.height())
    }
    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }
    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
    #[inline]
    pub fn intersects(&self, o: &Rect) -> bool {
        !(self.max.x < o.min.x || self.max.y < o.min.y || o.max.x < self.min.x || o.max.y < self.min.y)
    }
    #[inline]
    pub fn contains(&self, pos: Vect) -> bool {
        self.max.x > pos.x && self.min.x < pos.x && self.max.y > pos.y && self.min.y < pos.y
    }
    #[inline]
    pub fn fits_in(&self, o: &Rect) -> bool {
        self.max.x < o.max.x && self.max.y < o.max.y && o.min.x < self.min.x && o.min.y < self.min.y
    }
    #[inline]
    pub fn radius(&self) -> f32 {
        (self.max - self.max).len() / 2f32
    }

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