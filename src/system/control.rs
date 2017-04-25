use cgmath::Vector3;
use specs;
use world;

pub enum Event {
    MoveForward,
    MoveBackward,
    MoveRight,
    MoveLeft,
}

pub struct System {
    input: std::sync::mpsc::Receiver<Event>,
    move_speed: Vector3<f32>,
}

impl System {
    pub fn new(chan: std::sync::mpsc::Receiver<Event>) -> System {
        System {
            input: chan,
            accel: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.input.try_recv() {
                Ok(event) => match event {
                    Event::MoveForward  => move_speed + move_speed.unit_y(),
                    Event::MoveBackward => move_speed - move_speed.unit_y(),
                    Event::MoveRight    => move_speed + move_speed.unit_x(),
                    Event::MoveLeft     => move_speed - move_speed.unit_x(),
                },
                Err(_) => return, 
            }
        }
    }
}

impl specs::System<super::DeltaTime> for System {
    fn run(&mut self, arg: specs::RunArg, time: super::DeltaTime) {
        use specs::Join;

        self.check_input();

        let (mut control, mut transform) = arg.fetch(
            |w| (w.read::<world::Control>(), w.read::<world::Transform>())
        );

        for (control, transform) in (&control, &mut transform).iter() {
            let velocity = time * control.speed * self.accel;
            transform.pos = transform.pos + velocity;
        }
    }
}