use crate::math::vect::Vect;



#[derive(Copy, Clone)]
pub struct Mat {
    pub c: [f32; 6],
}

impl Mat {
    pub const  IM: Mat = Mat{ c:[1f32, 0f32, 0f32, 1f32, 0f32, 0f32]};
    pub const  ZM: Mat = Mat{ c:[0f32; 6]};

    #[inline]
    pub fn new(pos: Vect, scl: Vect, rot: f32) -> Mat {
        let s = rot.sin();
        let c = rot.cos();
        Mat{ c: [c * scl.x, s * scl.x, -s * scl.y, c * scl.y, pos.x, pos.y] }
    }

    #[inline]
    pub fn to_glm_mat4(&self) -> glm::Mat4 {
        return glm::Mat4::new(
            glm::Vec4::new(self.c[0], self.c[1], 0f32, 0f32),
            glm::Vec4::new(self.c[2], self.c[3], 0f32, 0f32),
            glm::Vec4::new(0f32, 0f32, 1f32, 0f32),
            glm::Vec4::new(self.c[4], self.c[5], 0f32, 1f32),
        )
    }

    #[inline]
    pub fn x_axis(&self) -> Vect {
        Vect::new(self.c[0], self.c[1])
    }

    #[inline]
    pub fn y_axis(&self) -> Vect {
        Vect::new(self.c[2], self.c[3])
    }

    #[inline]
    pub fn origin(&self) -> Vect {
        Vect::new(self.c[4], self.c[5])
    }

    #[inline]
    pub fn decompose(&self) -> [Vect; 3] {
        [self.x_axis(), self.y_axis(), self.origin()]
    }

    #[inline]
    pub fn scale_origin_xy(mut self, view: Vect) -> Mat {
        self.c[4] *= view.x;
        self.c[5] *= view.y;

        self
    }


    // Moved moves everything by the delta vector.
    #[inline]
    pub fn moved(mut self, delta: Vect) -> Mat {
        self.mv(delta);
        self
    }

    // Moved moves everything by the delta vector.
    #[inline]
    pub fn mv(&mut self, delta: Vect) {
        self.c[4] += delta.x;
        self.c[5] += delta.y;
    }

    #[inline]
    pub fn sxy(&mut self, scl: Vect) {
        self.c[0] *= scl.x;
        self.c[1] *= scl.y;
        self.c[2] *= scl.x;
        self.c[3] *= scl.y;
        self.c[4] *= scl.x;
        self.c[5] *= scl.y;
    }

    #[inline]
    pub fn scaled_xy(mut self, around: Vect, scl: Vect) -> Mat {
        self.mv(around.inverted());
        self.sxy(scl);
        self.mv(around);

        self
    }

    #[inline]
    pub fn scaled(mut self, around: Vect, scl: f32) -> Mat {
        self.mv(around.inverted());
        self.sxy( Vect::new(scl, scl));
        self.mv(around);

        self
    }

    #[inline]
    pub fn rotated(mut self, around: Vect, ang: f32) -> Mat {
        self.mv(around.inverted());

        let c = ang.cos();
        let s = ang.sin();
        self = self.chained(&Mat{c:[c, s, -s, c, 0f32, 0f32]});

        self.mv(around);

        self
    }

    #[inline]
    pub fn chained(&self, o: &Mat) -> Mat {
        let mut chained = ZM.clone();
        chained.c[0] = o.c[0] * self.c[0] + o.c[2] * self.c[1];
        chained.c[1] = o.c[1] * self.c[0] + o.c[3] * self.c[1];
        chained.c[2] = o.c[0] * self.c[2] + o.c[2] * self.c[3];
        chained.c[3] = o.c[1] * self.c[2] + o.c[3] * self.c[3];
        chained.c[4] = o.c[0] * self.c[4] + o.c[2] * self.c[5] + o.c[4];
        chained.c[5] = o.c[1] * self.c[4] + o.c[3] * self.c[5] + o.c[5];

        chained
    }

    #[inline]
    pub fn prj(&self, u: Vect) -> Vect {
        Vect::new(self.c[0] * u.x + self.c[2] * u.y + self.c[4], self.c[1] * u.x + self.c[3] * u.y + self.c[5])
    }

    #[inline]
    pub fn unprj(&self, u: Vect) -> Vect {
        let det = self.c[0] * self.c[3] - self.c[2] * self.c[1];
        Vect::new(
            (self.c[3] * (u.x - self.c[4]) - self.c[2] * (u.y - self.c[5])) / det,
            (-self.c[1] * (u.x - self.c[4]) + self.c[0] * (u.y - self.c[5])) / det,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::math::{mat, vect};
    use crate::math::vect::*;

    fn round(a: f32, decimals: i32) -> f32 {
        let mul = 10f32.powi(decimals);
        (a * mul).round() / mul
    }

    #[test]
    fn prj_test() {
        assert_eq!(Vect::ZERO, super::IM.prj(Vect::ZERO))
    }
}