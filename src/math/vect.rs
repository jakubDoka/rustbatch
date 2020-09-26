use std::ops;
use std::slice::Iter;
use crate::math::{clamp, clamp_f};

macro_rules! vec_from {
    ($arg:ident) => {
        handle!($arg);
    };

    ($arg:ident, $($args:ident),+) => (
        handle!($arg);
        vec_from!($($args),+);
    );
}

macro_rules! handle {
    ($arg:ident) => {
        handle!($arg; $arg);
    };

    ($name:ident; $type:ty) => {
        #[inline]
        pub fn $name(x: $type, y: $type) -> Vect {
            Vect{x: x as f32, y: y as f32}
        }
    };
}

/// Vect is 2D vector and is used all over the place. I choose to use f32 because
/// opengl also accepts only f32
#[derive(Copy, Clone, Debug)]
pub struct Vect{
    pub x: f32,
    pub y: f32,
}

impl Vect{
    pub const ZERO: Self = Self{x:0f32, y:0f32};
    pub const LEFT: Self = Self{x:-1f32, y:0f32};
    pub const RIGHT: Self = Self{x:1f32, y:0f32};
    pub const UP: Self = Self{x:0f32, y:1f32};
    pub const DOWN: Self = Self{x:0f32, y:-1f32};
    pub const MAX: Self = Self{x: f32::MAX, y: f32::MAX};
    pub const MIN: Self = Self{x: f32::MIN, y: f32::MIN};

    /// average returns average of slice of vectors
    #[inline]
    pub fn average(arr: &[Vect]) -> Self {
        let len =  arr.len();
        if len == 0 {
            return Self::ZERO;
        }

        let mut total = Self::ZERO;
        for vec in arr {
            total += *vec
        }

        total / len as f32
    }

    /// new is vector constructor
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self{x, y}
    }

    /// mirror returns homogenous vector
    #[inline]
    pub fn mirror(m: f32) -> Self {
        Self{x:m, y:m}
    }

    /// unit returns vector of length 1.0 with given angle
    #[inline]
    pub fn unit(a: f32) -> Self {
        Self{x:a.cos(), y:a.sin()}
    }

    /// rad is same as unit but you can also specify length
    #[inline]
    pub fn rad(a: f32, l: f32) -> Self {
        Self::unit(a) * l
    }

    /// clamped clamps a vectors length
    #[inline]
    pub fn clamped(&self, min: f32, max: f32) -> Self {
        Self::rad(self.ang(), clamp_f(self.len(), min, max))
    }

    /// ang returns vectors angle
    #[inline]
    pub fn ang(&self) -> f32 {
        self.y.atan2(self.x)
    }

    /// len returns vectors length
    #[inline]
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// norm returns normalized vector with length 1.0
    #[inline]
    pub fn norm(self) -> Self {
        let len = self.len();
        if len == 0f32 {
            return Self::ZERO
        }
        self/len
    }

    /// swp swaps x and y of vector
    #[inline]
    pub fn swp(self) -> Self {
        Self{x:self.y, y:self.x}
    }

    /// rot rotates vector by a
    #[inline]
    pub fn rot(self, a: f32) ->Self {
        Self::rad(self.ang() + a, self.len())
    }

    /// dist returns distance to other vector
    #[inline]
    pub fn dist(self, b: Self) -> f32 {
        (self - b).len()
    }

    /// to returns vector from self to b
    #[inline]
    pub fn to(self, b: Self) -> Self {
        b - self
    }

    /// dot returns vectors dot
    #[inline]
    pub fn dot(self, b: Self) -> f32 {
        self.x * b.x + self.y * b.y
    }

    /// ang_to returns smallest angle between two vectors
    #[inline]
    pub fn ang_to(self, b: Self) -> f32 {
        self.norm().dot( b.norm()).acos()
    }

    /// trn applies closure to both x and y of a vector
    #[inline]
    pub fn trn<T:Fn(f32) -> f32>(&self, tr: T) -> Self {
        Self{x:tr(self.x), y:tr(self.y)}
    }

    /// inverted inverts vector
    #[inline]
    pub fn inverted(&self) -> Self {
        Self{x: -self.x, y: -self.y}
    }

    vec_from!(u8, u16, u32, u64, i8, i16, i32, i64, i128, f64);
 }

impl std::cmp::PartialEq for Vect {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl ops::Add<Vect> for Vect {
    type Output = Vect;
    #[inline]
    fn add(self, rhs: Vect) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::AddAssign<Vect> for Vect {
    #[inline]
    fn add_assign(&mut self, rhs: Vect) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Vect> for Vect {
    type Output = Vect;
    #[inline]
    fn sub(self, rhs: Vect) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::SubAssign<Vect> for Vect {
    #[inline]
    fn sub_assign(&mut self, rhs: Vect) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<Vect> for Vect {
    type Output = Vect;
    #[inline]
    fn mul(self, rhs: Vect) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl ops::MulAssign<Vect> for Vect {
    #[inline]
    fn mul_assign(&mut self, rhs: Vect) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::Mul<f32> for Vect {
    type Output = Vect;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::MulAssign<f32> for Vect {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Div<Vect> for Vect {
    type Output = Vect;
    #[inline]
    fn div(self, rhs: Vect) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl ops::DivAssign<Vect> for Vect {
    #[inline]
    fn div_assign(&mut self, rhs: Vect) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl ops::Div<f32> for Vect {
    type Output = Vect;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl ops::DivAssign<f32> for Vect {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::math::vect::Vect;

    fn round(a: f32, decimals: i32) -> f32 {
        let mul = 10f32.powi(decimals);
        (a * mul).round() / mul
    }

    #[test]
    fn angle_test() {
        assert_eq!(PI, Vect::LEFT.ang())
    }
    #[test]
    fn ang_to_test() {
        assert_eq!(PI, Vect::LEFT.ang_to(Vect::RIGHT))
    }
    #[test]
    fn rot_test() {
        assert_eq!(Vect::LEFT.x,round(Vect::RIGHT.rot(PI).x, 6));
        assert_eq!(Vect::LEFT.y,round(Vect::RIGHT.rot(PI).y, 6));
    }
    #[test]
    fn average_test() {
        let vec = vec![Vect::LEFT, Vect::RIGHT];
        assert_eq!(Vect::average(&vec), Vect::ZERO);
    }
}