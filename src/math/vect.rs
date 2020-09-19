use std::ops;

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

#[derive(Copy, Clone, Debug)]
pub struct Vect{
    pub x: f32,
    pub y: f32,
}

pub static ZERO: Vect = Vect{x:0f32, y:0f32};
pub static LEFT: Vect = Vect{x:-1f32, y:0f32};
pub static RIGHT: Vect = Vect{x:1f32, y:0f32};
pub static UP: Vect = Vect{x:0f32, y:1f32};
pub static DOWN: Vect = Vect{x:0f32, y:-1f32};

impl Vect{
    #[inline]
    pub fn new(x: f32, y: f32) -> Vect {
        Vect{x, y}
    }

    #[inline]
    pub fn mirror(m: f32) -> Vect {
        Vect{x:m, y:m}
    }

    #[inline]
    pub fn unit(a: f32) -> Vect {
        Vect{x:a.cos(), y:a.sin()}
    }

    #[inline]
    fn rad(a: f32, l: f32) -> Vect {
        Self::unit(a) * l
    }

    #[inline]
    pub fn ang(&self) -> f32 {
        self.y.atan2(self.x)
    }

    #[inline]
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    #[inline]
    pub fn norm(self) -> Vect {
        self/self.len()
    }

    #[inline]
    pub fn swp(self) -> Vect {
        Vect{x:self.y, y:self.x}
    }

    #[inline]
    pub fn rot(self, a: f32) ->Vect {
        Self::rad(self.ang() + a, self.len())
    }

    #[inline]
    pub fn dist(self, b: Vect) -> f32 {
        (self - b).len()
    }

    #[inline]
    pub fn to(self, b: Vect) -> Vect {
        b - self
    }

    #[inline]
    pub fn dot(self, b: Vect) -> f32 {
        self.x * b.x + self.y * b.y
    }

    #[inline]
    pub fn ang_to(self, b: Vect) -> f32 {
        (self.dot( b)/(self.len()*b.len())).acos()
    }

    #[inline]
    pub fn trn<T:Fn(f32) -> f32>(&self, tr: T) -> Vect {
        Vect{x:tr(self.x), y:tr(self.y)}
    }

    #[inline]
    pub fn inverted(&self) -> Vect {
        Vect{x: -self.x, y: -self.y}
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

    fn round(a: f32, decimals: i32) -> f32 {
        let mul = 10f32.powi(decimals);
        (a * mul).round() / mul
    }

    #[test]
    fn angle_test() {
        assert_eq!(PI, super::LEFT.ang())
    }
    #[test]
    fn ang_to_test() {
        assert_eq!(PI, super::LEFT.ang_to(super::RIGHT))
    }
    #[test]
    fn rot_test() {
        assert_eq!(super::LEFT.x,round(super::RIGHT.rot(PI).x, 8));
        assert_eq!(super::LEFT.y,round(super::RIGHT.rot(PI).y, 8));
    }
}