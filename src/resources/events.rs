use glutin;
use specs;
use std::collections::HashSet;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub enum EventTypes {
    Closed,
    WindowResize(u32, u32),
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

#[derive(Clone, Debug)]
pub struct Events {
    events: Vec<EventTypes>,
    keyboard_map: HashSet<glutin::VirtualKeyCode>,
}

impl Events {
    pub fn new() -> Events {
        Events {
            events: Vec::new(),
            keyboard_map: HashSet::new(),
        }
    }

    pub fn event_callback(&mut self, event: glutin::Event) {
        match event {
            // Don't care about which window event comes from since we only have one window
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => self.add_event(EventTypes::Closed),
                    glutin::WindowEvent::Resized(x, y) => {
                        self.add_event(EventTypes::WindowResize(x, y))
                    }
                    glutin::WindowEvent::KeyboardInput(glutin::ElementState::Pressed, _, Some(keycode), _) => {
                        self.keyboard_map.insert(keycode);
                    }
                    glutin::WindowEvent::KeyboardInput(glutin::ElementState::Released, _, Some(keycode), _) => {
                        self.keyboard_map.remove(&keycode);
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn add_event(&mut self, event: EventTypes) {
        self.events.push(event);
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn dispatch_events(&mut self) {
        let current_key_state = self.keyboard_map.clone();
        for key in current_key_state {
            match map_key_to_event(key) {
                Some(e) => self.add_event(e),
                None => (),
            }
        }
    }
}

fn map_key_to_event(key: glutin::VirtualKeyCode) -> Option<EventTypes> {
    match key {
        glutin::VirtualKeyCode::W => Some(EventTypes::MoveUp),
        glutin::VirtualKeyCode::A => Some(EventTypes::MoveLeft),
        glutin::VirtualKeyCode::S => Some(EventTypes::MoveDown),
        glutin::VirtualKeyCode::D => Some(EventTypes::MoveRight),
        _ => None,
    }
}

pub fn update_events(event_loop: &glutin::EventsLoop, world: &mut specs::World) {
    // Get a mutable reference to the events
    let mut event_mut = world.write_resource::<Events>();
    // Clear previous events
    event_mut.clear();
    // Handle window events and update keymap
    event_loop.poll_events(|event| event_mut.event_callback(event));
    // Flush keymap to event storage
    event_mut.dispatch_events();
}

impl Deref for Events {
    type Target = Vec<EventTypes>;

    fn deref(&self) -> &Self::Target {
        &self.events
    }
}

