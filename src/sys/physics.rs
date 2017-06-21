use cgmath::Vector3;
use components;
use specs::{Join, ReadStorage, System, WriteStorage};

// Translates physical forces into transforms
pub struct PhysicsSys;

impl<'a> System<'a> for PhysicsSys {
    type SystemData = (ReadStorage<'a, components::Physics>, WriteStorage<'a, components::Transform>);

    fn run(&mut self, data: Self::SystemData) {
        let (physics, mut transforms) = data;

        for (p, t) in (&physics, &mut transforms).join() {
            t.position += p.velocity * 0.01;
        }
    }
}