use cgmath::Vector3;
use specs;

// Defines Transformation component
#[derive(Clone, Debug)]
pub struct Transform {
    pub pos: Vector3<f32>,
}

impl specs::Component for Transform {
    type Storage = specs::VecStorage<Transform>;
}

// Allows entities with this component to be controlled though events
#[derive(Clone, Debug)]
pub struct Control {
    pub velocity: Vector3<f32>,
}

impl specs::Component for Control {
    type Storage = specs::HashMapStorage<Control>;
}
