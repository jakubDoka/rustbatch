pub mod curve;
pub mod base;
pub mod rgba;
pub mod mat;
pub mod rect;
pub mod vect;

#[inline]
pub fn clamp<T: Ord>(a: T,min: T, max: T ) -> T {
    if a < min {
        return min
    } else if a > max {
        return max
    }
    a
}

#[inline]
pub fn clamp_f(a: f32,min: f32, max: f32 ) -> f32 {
    if a < min {
        return min
    } else if a > max {
        return max
    }
    a
}