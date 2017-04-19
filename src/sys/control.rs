use std::sync::mpsc;
use cgmath;
use specs;
use world;

pub enum Event {
    EvMove(cgmath::vec3<f32>),
}

pub struct System {
    input: mpsc::Receiver<Event>,
    position: cgmath::Vector3<f32>,
    velocity: cgmath::Vector3<f32>,
}

impl System {
    pub fn new(chan: mpsc::Receiver<Event>) -> System {
        System {
            input: chan,
            position: cgmath::vec3(0.0, 0.0, 0.0),
            velocity: cgmath::vec3(0.0, 0.0, 0.0),
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.input.try_recv() {
                Ok(Event::EvMove(accel)) => self.velocity = velocity + accel,
                Err(_) => return,
            }
        }
    }
}

impl specs::System<super::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, time: super::Delta) {
        use specs::Join;
        self.check_input();
        unimplemented!()
    }
}