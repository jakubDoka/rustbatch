use std::time::Instant;

pub struct FPS {
    counter: u16,
    timeout: f32,
    time: f32,
    last: Instant,
}

impl FPS {
    pub fn new(timeout: f32) -> Self {
        FPS{counter: 0u16, timeout, time: 0f32, last: Instant::now()}
    }

    pub fn increase(&mut self, mut delta: f32) -> f32 {
        if delta <= 0f32 {
            delta = Instant::elapsed(&self.last).as_secs_f32();
            self.last = Instant::now();
        }
        self.time += delta;
        self.counter += 1;
        if self.time > self.timeout {
            println!("fps:{}", self.counter as f32 / self.timeout);
            self.time = 0f32;
            self.counter = 0;
        }

        delta
    }
}