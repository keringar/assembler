use cgmath::{Vector3, Quaternion};
use specs;

// Defines Transformation component
#[derive(Clone, Debug)]
pub struct Transform {
    pub pos: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: f32,
}

impl specs::Component for Transform {
    type Storage = specs::VecStorage<Transform>;
}

// Physics allows entities to move around
#[derive(Clone, Debug)]
pub struct Inertial {
    pub velocity: Vector3<f32>,
}

impl specs::Component for Inertial {
    type Storage = specs::VecStorage<Inertial>;
}

// Allows entities with this component to be controlled though events
#[derive(Clone, Debug)]
pub struct Control {
    pub acceleration: f32,
}

impl specs::Component for Control {
    type Storage = specs::HashMapStorage<Control>;
}
