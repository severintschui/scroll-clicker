use std::cell::Cell;
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant};
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

struct Toggle {
    toggled: Cell<bool>,
    max_duration: Duration,
    previous: Cell<Option<Instant>>,
}

impl Toggle {
    fn new() -> Self {
        Self {
            toggled: Cell::new(false),
            max_duration: Duration::from_millis(200),
            previous: Cell::new(None),
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

    fn track_double_press(&self) {
        match self.previous.get() {
            None => self.previous.set(Some(Instant::now())),
            Some(previous) => {
                match previous.elapsed() <= self.max_duration {
                    true => {
                        self.previous.set(None);
                        self.toggle();
                    }
                    false => {
                        self.previous.set(Some(Instant::now()));
                    }
                }
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
        if let EventType::KeyPress(Key::ShiftLeft) = event.event_type {
            toggle.track_double_press();
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
    listener()
}
