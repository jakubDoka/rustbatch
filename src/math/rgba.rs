
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

#[inline]
pub fn lerp(a: &RGBA, b: &RGBA, t: f32) -> RGBA {
    let mut new = WHITE;
    for i in 0..COLOR_SIZE {
        new[i] = a[i] + (b[i]-a[i]) * t
    }

    new
}

#[derive(Copy, Clone)]
pub struct GraphPoint {
    point: f32,
    color: RGBA,
}

impl GraphPoint {
    #[inline]
    pub fn new(point: f32, color: RGBA) -> Self {
        if point < 0.0 || point > 1.0 {
            panic!("GraphPoint value is out of range. it has to be in inclusive range from 0 to 1 you supplied {}", point)
        }

        Self {
            point,
            color,
        }
    }
}

pub struct Graph {
    points: Vec<GraphPoint>,
}

impl Clone for Graph {
    fn clone(&self) -> Self {
        Self { points: self.points.clone() }
    }
}

impl Graph {
    #[inline]
    pub fn new(mut points: Vec<GraphPoint>) -> Self {
        let mut first = points[0];
        if first.point != 0.0 {
            first.point = 0.0;
            points.insert(0, first);
        }

        let mut last = points[points.len() - 1];
        if last.point != 1.0 {
            last.point = 1.0;
            points.push(last);
        }

        Self { points }
    }

    #[inline]
    pub fn get_color(&self, mut t: f32) -> RGBA {

        let mut idx = 0;
        for p in self.points.iter() {
            if p.point <= t {
                idx += 1
            }
        }

        let first = &self.points[idx-1];
        let second = &self.points[idx];
        lerp(&first.color, &second.color, (t - first.point) / (second.point - first.point))
    }
}