use cgmath;
use cgmath::prelude::*;
use components;
use events;
use specs::prelude::*;

pub struct Control;

impl<'a> System<'a> for Control {
    type SystemData = (Fetch<'a, events::Events>,
     ReadStorage<'a, components::Control>,
     WriteStorage<'a, components::Physics>);

    fn run(&mut self, data: Self::SystemData) {
        let (events, control, mut physics) = data;

        let mut accel_dir = cgmath::Vector2::new(0f32, 0f32);
        for event in (*events).iter() {
            match *event {
                events::EventTypes::MoveUp => accel_dir += cgmath::Vector2::unit_y(),
                events::EventTypes::MoveDown => accel_dir -= cgmath::Vector2::unit_y(),
                events::EventTypes::MoveLeft => accel_dir += cgmath::Vector2::unit_y(),
                events::EventTypes::MoveRight => accel_dir -= cgmath::Vector2::unit_y(),
                _ => (),
            }
        }
        accel_dir.normalize();

        for (c, p) in (&control, &mut physics).join() {
            p.velocity += accel_dir * (c.force / p.mass);
            println!("Increasing velocity: {:?}", p.velocity);
        }
    }
}
