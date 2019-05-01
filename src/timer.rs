use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::time::{Duration, Instant};

// is this even the best way to do this??
lazy_static! {
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

pub fn restart_timer() {
    TIMER.lock().start = Instant::now();
}

pub fn elapsed() -> Duration {
    TIMER.lock().start.elapsed()
}
