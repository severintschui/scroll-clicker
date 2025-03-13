use std::cell::Cell;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Throttle<T, F: Fn() -> T> {
    f: F,
    min_duration: Duration,
    previous: Cell<Instant>,
}

impl<T, F: Fn() -> T> Throttle<T, F> {
    pub fn new(f: F, min_duration: Duration) -> Self {
        let previous = Cell::new(Instant::now() - min_duration);
        Self {
            f,
            min_duration,
            previous,
        }
    }

    pub fn call(&self) -> Option<T> {
        let now = Instant::now();
        let previous = self.previous.get();

        if now.duration_since(previous) >= self.min_duration {
            self.previous.set(now);
            Some((self.f)())
        } else {
            None
        }
    }
}
