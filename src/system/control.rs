use std::sync::mpsc;
use cgmath::{Vector3, Zero};

use specs;
use util::types::*;
use world;

pub enum Event {
    MoveForward,
    MoveBackward,
    MoveRight,
    MoveLeft,
}

pub struct System {
    input_chan: mpsc::Receiver<Event>,
    accel_dir: Vector3<f32>,
}

impl System {
    pub fn new(input_chan: mpsc::Receiver<Event>) -> System {
        System {
            input_chan,
            accel_dir: Vector3::zero(),
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.input_chan.try_recv() {
                Ok(event) => {
                    match event {
                        Event::MoveForward => self.accel_dir = Vector3::unit_y(),
                        Event::MoveBackward => self.accel_dir = -Vector3::unit_y(),
                        Event::MoveRight => self.accel_dir = Vector3::unit_x(),
                        Event::MoveLeft => self.accel_dir = -Vector3::unit_x(),
                    }
                }
                Err(_) => return,
            }
        }
    }
}

impl specs::System<DeltaTime> for System {
    fn run(&mut self, arg: specs::RunArg, time: DeltaTime) {
        use specs::Join;

        self.check_input();

        let (mut control, mut transform) =
            arg.fetch(|w| (w.write::<world::Control>(), w.write::<world::Transform>()),);

        for (control, transform) in (&mut control, &mut transform).join() {
            control.velocity = control.velocity + (time * self.accel_dir);
            transform.pos = transform.pos + control.velocity;
        }
    }
}
