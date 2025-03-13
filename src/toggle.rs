use std::cell::{Cell, RefCell};
use std::time::{Duration, Instant};

use circular_buffer::CircularBuffer;
use rdev::{EventType, Key};

#[derive(Debug)]
pub struct Toggle {
    toggled: Cell<bool>,
    max_duration: Duration,
    buffer: RefCell<CircularBuffer<4, TimedKeyEvent>>,
}

impl Toggle {
    pub fn new() -> Self {
        Self {
            toggled: Cell::new(false),
            max_duration: Duration::from_millis(250),
            buffer: RefCell::new(CircularBuffer::new()),
        }
    }

    fn toggle(&self) {
        match self.toggled.replace(!self.toggled.get()) {
            true => println!("Untoggled"),
            false => println!("Toggled"),
        }
    }

    pub fn is_toggled(&self) -> bool {
        self.toggled.get()
    }

    pub fn track_double_press(&self, event_type: EventType) {
        let mut buffer = self.buffer.borrow_mut();
        buffer.push_back(TimedKeyEvent::new(event_type));

        if let (
            Some(TimedKeyEvent {
                event_type: EventType::KeyPress(Key::ShiftLeft),
                time: start,
            }),
            Some(TimedKeyEvent {
                event_type: EventType::KeyRelease(Key::ShiftLeft),
                time: _,
            }),
            Some(TimedKeyEvent {
                event_type: EventType::KeyPress(Key::ShiftLeft),
                time: _,
            }),
            Some(TimedKeyEvent {
                event_type: EventType::KeyRelease(Key::ShiftLeft),
                time: end,
            }),
        ) = (buffer.get(0), buffer.get(1), buffer.get(2), buffer.get(3))
        {
            if end.duration_since(*start) <= self.max_duration {
                buffer.clear();
                self.toggle();
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TimedKeyEvent {
    event_type: EventType,
    time: Instant,
}

impl TimedKeyEvent {
    fn new(event_type: EventType) -> Self {
        Self {
            event_type,
            time: Instant::now(),
        }
    }
}
