use std::sync::mpsc;
use glutin;
use system;

pub struct ReceiverHub {
    pub control: mpsc::Receiver<sys::control::Event>,
}

pub struct SenderHub {
    control: mpsc::Sender<system::control::Event>,
}

impl SenderHub {
    pub fn new() -> (SenderHub, ReceiverHub) {
        let (control_send, control_recv) = mpsc::channel();
        
        let send_hub = SenderHub {
            control: control_send,
        }

        let recv_hub = ReceiverHub {
            control: control_recv,
        }

        (send_hub, recv_hub)
    }

    pub fn process_glutin(&self, event: glutin::Event) {
        use glutin::Event::KeyboardInput;
        use glutin::{ElementState, VirtualKeyCode};

        use system::control::Event;

        match event {
            KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) => self.control.send( Event::MoveEvent ).unwrap(),
            _ => (),
        }
    }
}