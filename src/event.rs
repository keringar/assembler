use glutin;
use std::ops::Deref;
use bus::{Bus, BusReader, self};

#[derive(Debug, Copy, Clone)]
pub enum Event {
    MoveForward,
    MoveLeft,
    MoveRight,
    MoveBackward,
    Quit,
    NotHandled,
}

pub struct EventManager {
    event_loop: glutin::EventsLoop,
    bus: BusWrapper,
}

// Required to get around partial borrows
struct BusWrapper {
    pub bus: Bus<Event>,
}

impl EventManager {
    pub fn new() -> EventManager {
        // TODO: Profile to find optimal event queue size
        // for now, just set it at 100 bytes. Each event
        // is one byte.
        let bus = BusWrapper {  
            bus: Bus::new(100),
        };

        EventManager {
            event_loop: glutin::EventsLoop::new(),
            bus,
        }
    }

    pub fn poll_events(&mut self) {
        let bus = &mut self.bus;

        self.event_loop
            .poll_events(
                |event| match event {
                    // Contains event and the windowID that created it.
                    // WindowID is ignored as we only have one window
                    glutin::Event::WindowEvent { event, .. } => {
                        match event {
                            glutin::WindowEvent::Closed => bus.bus.broadcast(Event::Quit),
                            glutin::WindowEvent::KeyboardInput(_, _, Some(keycode), _) => {
                                let event = map_key(keycode);
                                bus.bus.broadcast(event);
                            }
                            _ => (),
                        }
                    }
                },
            );
    }

    // Example:
    //  let mut receiver = events.add_listener();
    //  std::thread::spawn(move || {
    //      loop {
    //          let event = receiver.recv().unwrap();
    //  
    //          match event {
    //              _ => (),
    //          }
    //      }
    //  });
    pub fn add_listener(&mut self) -> BusReader<Event> {
        self.bus.bus.add_rx()
    }
}

impl Deref for EventManager {
    type Target = glutin::EventsLoop;

    fn deref(&self) -> &glutin::EventsLoop {
        &self.event_loop
    }
}

fn map_key(keycode: glutin::VirtualKeyCode) -> Event {
    match keycode {
        glutin::VirtualKeyCode::W => Event::MoveForward,
        glutin::VirtualKeyCode::A => Event::MoveLeft,
        glutin::VirtualKeyCode::S => Event::MoveBackward,
        glutin::VirtualKeyCode::D => Event::MoveRight,
        _ => Event::NotHandled,
    }
}
