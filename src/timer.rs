use lazy_static::lazy_static;
use std::time::{Instant, Duration};
use std::sync::Mutex;

// is this even the best way to do this??
lazy_static!{
    static ref TIMER: Mutex<Stopwatch> = Mutex::new(Stopwatch::new());
}

struct Stopwatch {
    start: Instant,
}

impl Stopwatch {
    fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}


pub fn start_timer() {
    TIMER.lock().unwrap().start = Instant::now();
}

pub fn elapsed() -> Duration {
    TIMER.lock().unwrap().start.elapsed()
}
