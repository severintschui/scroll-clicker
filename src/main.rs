use std::cell::{Cell, RefCell};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant};
use circular_buffer::CircularBuffer;
use rdev::{grab, simulate, Button, Event, EventType, Key};

struct Throttle<F> {
    f: F,
    min_duration: Duration,
    previous: Cell<Instant>,
}

impl <T, F: Fn() -> T> Throttle<F> {
    fn new(f: F, min_duration: Duration) -> Self {
        let previous = Cell::new(Instant::now() - min_duration);
        Self {
            f,
            min_duration,
            previous,
        }
    }

    fn call(&self) -> Option<T> {
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

struct Toggle {
    toggled: Cell<bool>,
    max_duration: Duration,
    buffer: RefCell<CircularBuffer<4, TimedKeyEvent>>,
}

impl Toggle {
    fn new() -> Self {
        Self {
            toggled: Cell::new(false),
            max_duration: Duration::from_millis(250),
            buffer: RefCell::new(CircularBuffer::new()),
        }
    }

    fn toggle(&self) {
        let previous = self.toggled.get();
        self.toggled.set(!previous);
        match previous {
            true => println!("Untoggled"),
            false => println!("Toggled"),
        }
    }

    fn is_toggled(&self) -> bool {
        self.toggled.get()
    }

    fn track_double_press(&self, event_type: EventType) {
        let mut buffer = self.buffer.borrow_mut();
        buffer.push_back(TimedKeyEvent::new(event_type));

        if let (
            Some(TimedKeyEvent { event_type: EventType::KeyPress(Key::ShiftLeft), time: start }),
            Some(TimedKeyEvent { event_type: EventType::KeyRelease(Key::ShiftLeft), time: _ }),
            Some(TimedKeyEvent { event_type: EventType::KeyPress(Key::ShiftLeft), time: _ }),
            Some(TimedKeyEvent { event_type: EventType::KeyRelease(Key::ShiftLeft), time: end }),
        ) = (
            buffer.get(0),
            buffer.get(1),
            buffer.get(2),
            buffer.get(3)
        ) {
            if end.duration_since(*start) <= self.max_duration {
                buffer.clear();
                self.toggle();
            }
        }
    }
}

struct MouseClickEvent;

fn listener() {
    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        while let Ok(_) = rx.recv() {
            simulate(&EventType::ButtonPress(Button::Left)).unwrap();
            thread::sleep(Duration::from_millis(1));
            simulate(&EventType::ButtonRelease(Button::Left)).unwrap();
        }
    });

    let mouse_click = Throttle::new(move ||{
        tx.send(MouseClickEvent).unwrap();
    }, Duration::from_millis(50));

    let toggle = Toggle::new();

    let callback = move |event: Event| -> Option<Event> {
        if let EventType::KeyPress(Key::ShiftLeft) | EventType::KeyRelease(Key::ShiftLeft) = event.event_type {
            toggle.track_double_press(event.event_type);
        }

        if toggle.is_toggled() {
            if let EventType::Wheel { delta_x: 0, delta_y: _ }  = event.event_type {
                mouse_click.call();
                return None;
            }
        }

        Some(event)
    };

    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error);
    }

    handle.join().unwrap();
}

fn main() {
    // foo();
    listener()
}
