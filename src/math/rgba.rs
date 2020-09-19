
pub type RGBA = [f32; 4];

// the number syntax is shit
static O: f32 = 1f32;
static N: f32 = 0f32;

pub static WHITE: RGBA = [O, O, O, O];
pub static BLACK: RGBA = [N, N, N, O];

pub static COLOR_SIZE: usize = 4;

pub fn invert(color: &RGBA) -> RGBA {
    let mut new = WHITE;
    for i in 0..COLOR_SIZE - 1 {
        new[i] -= color[i];
    }

    new
}

pub fn mul(a: &RGBA, b: &RGBA) -> RGBA {
    let mut new = WHITE;
    for i in 0..COLOR_SIZE {
        new[i] = a[i] * b[i];
    }

    new
}
