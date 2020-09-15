extern crate procedures;
use procedures::repeat;

repeat!(fn goodT(t: T) -> T {
t
} for_ f64);

fn main() {
    println!("{}", goodf64(0f64))
}
