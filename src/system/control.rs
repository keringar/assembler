use cgmath::{Vector3, Zero};
use event::{Event, EventReceiver};
use specs;
use util::types::*;
use world;

pub struct System {
    input_chan: EventReceiver,
    accel_dir: Vector3<f32>,
}

impl System {
    pub fn new(input_chan: EventReceiver) -> System {
        System {
            input_chan,
            accel_dir: Vector3::zero(),
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.input_chan.recv() {
                Ok(event) => {
                    match event {
                        Event::MoveForward => self.accel_dir = Vector3::unit_y(),
                        Event::MoveBackward => self.accel_dir = -Vector3::unit_y(),
                        Event::MoveRight => self.accel_dir = Vector3::unit_x(),
                        Event::MoveLeft => self.accel_dir = -Vector3::unit_x(),
                        _ => (),
                    }
                }
                Err(_) => break,
            }
        }
    }
}

impl specs::System<DeltaTime> for System {
    fn run(&mut self, arg: specs::RunArg, time: DeltaTime) {
        use specs::Join;
        println!("Before");
        self.check_input();
        println!("After");
        let (mut inertia, control) =
            arg.fetch(|w| (w.write::<world::Inertial>(), w.read::<world::Control>()),);

        for (i, c) in (&mut inertia, &control).join() {
            i.velocity = i.velocity + (time * self.accel_dir * c.acceleration);
        }
    }
}
