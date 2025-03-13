mod throttle;
mod toggle;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use rdev::{grab, simulate, Button, Event, EventType, Key};

use throttle::Throttle;
use toggle::Toggle;

struct MouseClickTrigger;

fn listener() {
    let (tx, rx) = channel();

    // for Windows, it's necessary to simulate events in another thread
    // otherwise, the application will lag out when sending many events
    let handle = thread::spawn(move || {
        while let Ok(_) = rx.recv() {
            simulate(&EventType::ButtonPress(Button::Left)).unwrap();
            thread::sleep(Duration::from_millis(1));
            simulate(&EventType::ButtonRelease(Button::Left)).unwrap();
        }
    });

    let mouse_click = Throttle::new(move ||{
        tx.send(MouseClickTrigger).unwrap();
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
    listener()
}
