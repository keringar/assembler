use cgmath::Vector2;
use cgmath::prelude::*;
use specs::prelude::*;

// Allows entities with this component to be controlled though events
#[derive(Clone, Debug)]
pub struct Control {
    pub force: f32,
}

impl Component for Control {
    type Storage = HashMapStorage<Control>;
}

#[derive(Clone, Debug)]
pub struct Physics {
    pub velocity: Vector2<f32>,
    pub mass: f32,
}

impl Component for Physics {
    type Storage = VecStorage<Physics>;
}
