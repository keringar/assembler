use cgmath::{Vector3, Quaternion};
use specs;

// Defines Transformation component
#[derive(Clone, Debug)]
pub struct Transform {
    pub pos: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: f32,
}

// Physics allows entities to move around
#[derive(Clone, Debug)]
pub struct Inertial {
    pub velocity: Vector3<f32>,
}

// Allows entities with this component to be controlled though events
#[derive(Clone, Debug)]
pub struct Control {
    pub acceleration: f32,
}

#[derive(Clone, Debug)]
pub struct Renderable {
    //Renderable
}

#[derive(Clone, Debug)]
pub struct Chunk {
    //Render chunk
}

impl specs::Component for Transform {
    type Storage = specs::VecStorage<Transform>;
}

impl specs::Component for Inertial {
    type Storage = specs::VecStorage<Inertial>;
}

impl specs::Component for Control {
    type Storage = specs::HashMapStorage<Control>;
}

impl specs::Component for Renderable {
    type Storage = specs::VecStorage<Renderable>;
}

impl specs::Component for Chunk {
    type Storage = specs::HashMapStorage<Chunk>;
}