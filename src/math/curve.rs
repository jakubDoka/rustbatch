use crate::Vect;

#[derive(Copy, Clone)]
pub struct Curve {
    a: Vect,
    a_handle: Vect,
    b: Vect,
    b_handle: Vect,
    placeholder: bool
}

impl Curve {

    pub const NONE: Self = Self{a: Vect::ZERO, b: Vect::ZERO, a_handle: Vect::ZERO, b_handle: Vect::ZERO, placeholder: true };
    pub const LINEAR_INCREASING: Self = curve!(0, 0; 0, 0; 1, 1; 1, 1);
    pub const LINEAR_DECREASING: Self = curve!(0, 1; 0, 1; 1, 0; 1, 0);
    #[inline]
    pub fn new(a: (f32, f32), a_handle: (f32, f32), b: (f32, f32), b_handle: (f32, f32)) -> Self {
        Self {
            a: Vect {x: a.0, y: a.1},
            a_handle: Vect {x: a_handle.0, y: a_handle.1},
            b: Vect {x: b.0, y: b.1},
            b_handle: Vect {x: b_handle.0, y: b_handle.1},
            placeholder: false,
        }
    }

    #[inline]
    pub fn get_point(&self, t: f32) -> Vect {
        if self.placeholder {
            return Vect::UP;
        }

        let omt = (1.0 - t);

        self.a * (omt * omt * omt) + self.a_handle * (omt * omt * t * 3.0) + self.b_handle * (omt * t * t * 3.0) + self.b * (t * t * t)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::curve::Curve;
    use crate::Vect;

    #[test]
    fn curve_test() {
        println!("{:?}" ,Curve::LINEAR_INCREASING.get_point(0.5))
    }
}