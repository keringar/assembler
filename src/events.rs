use std::collections::HashSet;
use std::ops::Deref;
use glutin;

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
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => self.add_event(EventTypes::Closed),
                    glutin::WindowEvent::Resized(x, y) => {
                        self.add_event(EventTypes::WindowResize(x, y))
                    }
                    glutin::WindowEvent::KeyboardInput(glutin::ElementState::Pressed,
                                                       _,
                                                       Some(keycode),
                                                       _) => {
                        self.keyboard_map.insert(keycode);
                    }
                    glutin::WindowEvent::KeyboardInput(glutin::ElementState::Released,
                                                       _,
                                                       Some(keycode),
                                                       _) => {
                        self.keyboard_map.insert(keycode);
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

    pub fn flush(&mut self) {
        let pressed_keys: Vec<glutin::VirtualKeyCode> = self.keyboard_map.drain().collect();
        for key in pressed_keys {
            let event = map_key_to_event(key);
            match event {
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

impl Deref for Events {
    type Target = Vec<EventTypes>;

    fn deref(&self) -> &Self::Target {
        &self.events
    }
}
