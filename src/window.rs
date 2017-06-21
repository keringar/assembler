use glutin::{self, EventsLoop, WindowBuilder};
use std::ops::Deref;

pub struct Window {
    ev_loop: EventsLoop,
    window: glutin::Window,
}

impl Window {
    pub fn new() -> Window {
        let ev_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title("ECS")
            .build(&ev_loop)
            .expect("FATAL: Unable to create window");

        Window { ev_loop, window }
    }
}

impl Deref for Window {
    type Target = glutin::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

