use crate::vect::Vect;
use std::ops::Mul;

pub static DEFAULT_VIEWPORT_MATRIX: glm::Mat4 = glm::Mat4{
    c0: glm::Vec4{x: 1f32, y: 0f32, z: 0f32, w: 0f32},
    c1: glm::Vec4{x: 0f32, y: 1f32, z: 0f32, w: 0f32},
    c2: glm::Vec4{x: 0f32, y: 0f32, z: 1f32, w: 0f32},
    c3: glm::Vec4{x: -1f32, y: -1f32, z: 0f32, w: 1f32},
};

pub static IM: Mat = Mat{c: [1f32, 0f32, 0f32, 1f32, 0f32, 0f32]};

#[derive(Copy, Clone, Debug)]
pub struct Mat {
    c: [f32; 6]
}

impl Mat {
    pub fn new(pos: Vect, scl: Vect, rot: f32) -> Mat {
        let s = rot.sin();
        let c = rot.cos();
        Mat { c: [c * scl.x, s * scl.x, c * scl.y, s * scl.y, pos.x, pos.y] }
    }

    pub fn to_glm_mat4(&self) -> glm::Mat4 {
        return glm::Mat4::new(
            glm::Vec4::new(self.c[0], self.c[1], 0f32, 0f32),
            glm::Vec4::new(self.c[2], self.c[3], 0f32, 0f32),
            glm::Vec4::new(0f32, 0f32, 1f32, 0f32),
            glm::Vec4::new(self.c[4], self.c[5], 0f32, 1f32),
        )
    }

    pub fn xax(&self) -> Vect {
        Vect::new(self.c[0], self.c[1])
    }

    pub fn yax(&self) -> Vect {
        Vect::new(self.c[2], self.c[3])
    }

    pub fn org(&self) -> Vect {
        Vect::new(self.c[4], self.c[5])
    }

    // Moved moves everything by the delta vector.
    pub fn moved(mut self, delta: Vect) -> Mat {
        self.mv(delta);
        self
    }

    // Moved moves everything by the delta vector.
    pub fn mv(&mut self, delta: Vect) {
        self.c[4] += delta.x;
        self.c[5] += delta.y;
    }

    pub fn scl_a_xy(mut self, around: Vect, scl: Vect) -> Mat {
        self.mv(around.inverted());
        self.sxy(scl);
        self.mv(around);

        self
    }

    pub fn sxy(&mut self, scl: Vect) {
        self.c[0] *= scl.x;
        self.c[1] *= scl.y;
        self.c[2] *= scl.x;
        self.c[3] *= scl.y;
        self.c[4] *= scl.x;
        self.c[5] *= scl.y;
    }

    pub fn scl(mut self ,scl: f32) -> Mat {
        let pos = self.org();
        self.mv(pos.inverted());
        self.sxy(Vect::new(scl, scl));
        self.mv(pos);

        self
    }

    pub fn scl_a(mut self, around: Vect, scl: f32) -> Mat {
        self.scl_a_xy(around, Vect::new(scl, scl))
    }

    pub fn rot_a(mut self, around: Vect, ang: f32) -> Mat {
        self.mv(around.inverted());
        self.rt(ang);
        self.mv(around);

        self
    }

    pub fn rt(&mut self, ang: f32) {
        let c = ang.cos();
        let s = ang.sin();
        self.chn(Mat { c: [c, s, -s, c, 0f32, 0f32] })
    }

    pub fn chained(mut self, o: Mat) -> Mat {
        self.chn(o);
        self
    }

    pub fn chn(&mut self, o: Mat) {
        self.c[0] = o.c[0] * self.c[0] + o.c[2] * self.c[1];
        self.c[1] = o.c[1] * self.c[0] + o.c[3] * self.c[1];
        self.c[2] = o.c[0] * self.c[2] + o.c[2] * self.c[3];
        self.c[3] = o.c[1] * self.c[2] + o.c[3] * self.c[3];
        self.c[4] = o.c[0] * self.c[4] + o.c[2] * self.c[5] + o.c[4];
        self.c[5] = o.c[1] * self.c[4] + o.c[3] * self.c[5] + o.c[5];
    }

    pub fn prj(&self, u: Vect) -> Vect {
        Vect::new(self.c[0] * u.x + self.c[2] * u.y + self.c[4], self.c[1] * u.x + self.c[3] * u.y + self.c[5])
    }

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
    use crate::vect::*;
    use crate::vect;

    fn round(a: f32, decimals: i32) -> f32 {
        let mul = 10f32.powi(decimals);
        (a * mul).round() / mul
    }

    #[test]
    fn prj_test() {
        assert_eq!(vect::ZERO, super::IM.prj(vect::ZERO))
    }
}