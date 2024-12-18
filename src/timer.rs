use std::time::{Duration, Instant};

pub struct Timer {
    start_time: Instant,
}
impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start_time.elapsed();
        println!("Elapsed time: {:.2?}", elapsed);
    }
}