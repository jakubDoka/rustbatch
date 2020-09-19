pub mod rgba;
pub mod mat;
pub mod rect;
pub mod vect;

pub fn clamp<T: Ord>(a: T,min: T, max: T ) -> T {
    if a < min {
        return min
    } else if a > max {
        return max
    }
    a
}