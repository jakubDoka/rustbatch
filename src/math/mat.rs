use crate::math::vect::Vect;
use std::f32::consts::PI;


/// mat is 3 x 3 matrix used for 2D transformations. Simples explanation i can coe up with:
/// 3x3 matrix is vector trio where x is vector describing x axis and y is describing y axis
/// c is determining where these two axises intersect. Matrix is something like a custom coordinate
/// system. To convert point from classic coordinate system to matrix coordinates you use `prj`
/// method. Don't worry though if you do not understand. Matrix in this package is just optional
/// interface you may or may not use. All it does are slightly more complex transformations.
#[derive(Copy, Clone)]
pub struct Mat {
    pub x: Vect,
    pub y: Vect,
    pub c: Vect,
}

impl Mat {
    pub const IM: Mat = Mat{ x: Vect::RIGHT, y: Vect::UP, c:Vect::ZERO};
    pub const ZM: Mat = Mat{ x: Vect::ZERO, y: Vect::ZERO, c: Vect::ZERO};

    /// new is matrix constructor
    #[inline]
    pub fn new(pos: Vect, scl: Vect, rot: f32) -> Mat {
        Mat{
            x: Vect::rad(rot, scl.x),
            y: Vect::rad(rot + PI/2f32, scl.y),
            c: pos
        }
    }

    /// to_glm_mat4 turns Mat to glm::Mat4 for internal purposes
    #[inline]
    pub fn to_glm_mat4(&self) -> glm::Mat4 {
        return glm::Mat4::new(
            glm::Vec4::new(self.x.x, self.x.y, 0f32, 0f32),
            glm::Vec4::new(self.y.x, self.y.y, 0f32, 0f32),
            glm::Vec4::new(0f32, 0f32, 1f32, 0f32),
            glm::Vec4::new(self.c.x, self.c.y, 0f32, 1f32),
        )
    }

    #[inline]
    pub(crate) fn transform_from_window_space(mut self, size: (u32, u32)) -> Self {
        self.c /= Vect::u32(size.0, size.1)/2f32;
        self
    }

    // Moved moves everything by the delta vector.
    #[inline]
    pub fn moved(mut self, delta: Vect) -> Mat {
        self.c += delta;
        self
    }

    #[inline]
    pub fn sxy(&mut self, scl: Vect) {
        self.x *= scl;
        self.y *= scl;
        self.c *= scl;
    }

    #[inline]
    pub fn scaled_xy(mut self, around: Vect, scl: Vect) -> Mat {
        self.c -= around;
        self.sxy(scl);
        self.c += around;

        self
    }

    #[inline]
    pub fn scaled(mut self, around: Vect, scl: f32) -> Mat {
        self.c -= around;
        self.sxy( Vect::new(scl, scl));
        self.c += around;

        self
    }

    #[inline]
    pub fn rotated(mut self, around: Vect, ang: f32) -> Mat {
        self.c -= around;

        let c = ang.cos();
        let s = ang.sin();

        self = self.chained(&Mat{
            x: Vect {x: c, y: s},
            y: Vect {x: -s, y: c},
            c: Vect::ZERO
        });

        self.c += around;

        self
    }

    #[inline]
    pub fn chained(&self, o: &Mat) -> Mat {
        let mut chained = Self::ZM.clone();
        chained.x.x = o.x.x * self.x.x + o.y.x * self.x.y;
        chained.x.y = o.x.y * self.x.x + o.y.y * self.x.y;
        chained.y.x = o.x.x * self.y.x + o.y.x * self.y.y;
        chained.y.y = o.x.y * self.y.x + o.y.y * self.y.y;
        chained.c.x = o.x.x * self.c.x + o.y.x * self.c.y + o.c.x;
        chained.c.y = o.x.y * self.c.x + o.y.y * self.c.y + o.c.y;

        chained
    }

    #[inline]
    pub fn prj(&self, u: Vect) -> Vect {
        Vect::new(self.x.x * u.x + self.y.x * u.y + self.c.x, self.x.y * u.x + self.y.y * u.y + self.c.y)
    }

    #[inline]
    pub fn unprj(&self, u: Vect) -> Vect {
        let det = self.x.x * self.y.y - self.y.x * self.x.y;
        Vect::new(
            (self.y.y * (u.x - self.c.x) - self.y.x * (u.y - self.c.y)) / det,
            (-self.x.y * (u.x - self.c.x) + self.x.x * (u.y - self.c.y)) / det,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::math::{mat, vect};
    use crate::math::vect::*;
    use crate::Mat;

    fn round(a: f32, decimals: i32) -> f32 {
        let mul = 10f32.powi(decimals);
        (a * mul).round() / mul
    }

    #[test]
    fn prj_test() {
        assert_eq!(Vect::ZERO, Mat::IM.prj(Vect::ZERO))
    }
}