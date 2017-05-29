use bus::{Bus, BusReader};
use glutin;
use std::ops::Deref;

pub use std::sync::mpsc::{RecvError, TryRecvError};

#[derive(Debug, Copy, Clone)]
pub enum Event {
    MoveForward,
    MoveLeft,
    MoveRight,
    MoveBackward,
    Quit,
    Resize(u32, u32),
    NotHandled,
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

pub struct EventManager {
    event_loop: glutin::EventsLoop,
    bus: BusWrapper,
}

impl EventManager {
    pub fn new() -> (EventManager, EventReceiver) {
        // TODO: Profile to find optimal event queue size
        // for now, just set it at 100 bytes. Each event
        // is one byte.
        let bus = BusWrapper { bus: Bus::new(100) };

        let mut manager = EventManager {
            event_loop: glutin::EventsLoop::new(),
            bus,
        };

        let receiver = manager.add_receiver();

        (manager, receiver)
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
                            },
                            glutin::WindowEvent::Resized(x, y) => bus.bus.broadcast(Event::Resize(x, y)),
                            _ => (),
                        }
                    }
                },
            );
    }

    #[allow(dead_code)]
    pub fn dispatch(&mut self, event: Event) {
        self.bus.bus.broadcast(event)
    }

    pub fn add_receiver(&mut self) -> EventReceiver {
        EventReceiver { bus_receiver: self.bus.bus.add_rx() }
    }
}

impl Deref for EventManager {
    type Target = glutin::EventsLoop;

    fn deref(&self) -> &glutin::EventsLoop {
        &self.event_loop
    }
}

// No need to unnecessarily export implementation details
pub struct EventReceiver {
    bus_receiver: BusReader<Event>,
}

impl EventReceiver {
    // Bus should never drop before listeners, so no danger of getting a recv error
    #[allow(dead_code)]
    pub fn recv(&mut self) -> Result<Event, RecvError> {
        self.bus_receiver.recv()
    }

    // ^^
    pub fn try_recv(&mut self) -> Result<Event, TryRecvError> {
        self.bus_receiver.try_recv()
    }
}

// Required to get around partial borrows
struct BusWrapper {
    pub bus: Bus<Event>,
}
