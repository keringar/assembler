use cgmath::{InnerSpace, Vector3, Zero};
use components;
use resources::events::*;
use specs::{Fetch, Join, ReadStorage, System, WriteStorage};

pub struct ControlSys;

impl<'a> System<'a> for ControlSys {
    type SystemData = (Fetch<'a, Events>,
     ReadStorage<'a, components::Player>,
     WriteStorage<'a, components::Physics>);

    fn run(&mut self, data: Self::SystemData) {
        let (events, player, mut physics) = data;

        let mut accel_dir = Vector3::zero();
        for event in events.iter() {
            match *event {
                EventTypes::MoveUp => accel_dir += Vector3::unit_y(),
                EventTypes::MoveDown => accel_dir -= Vector3::unit_y(),
                EventTypes::MoveLeft => accel_dir -= Vector3::unit_x(),
                EventTypes::MoveRight => accel_dir += Vector3::unit_x(),
                _ => (),
            }
        }
        accel_dir.normalize();

        for (player, physics) in (&player, &mut physics).join() {
            physics.apply_force(accel_dir, player.force, 0.01);
        }
    }
}

