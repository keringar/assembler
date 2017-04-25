use cgmath::{Vector3}
use specs;
use system::render::DrawHandle;

// Defines Transformation component
#[derive(Clone)]
pub struct Transform {
    pub pos: Vector3<f32>,
}

impl specs::Component for Transform {
    type Storage = specs::VecStorage<Transform>;
}

// Defines drawable component, all renderable entities must have this
pub struct Sprite {
    // Mesh data
    slice: DrawHandle,
    //Shader param
    data: SpriteParam,
}

impl specs::Component for Sprite {
    type Storage = specs::VecStorage<Sprite>;
}

// Allows entities with this component to be controlled though events
pub struct Control {
    pub speed: f32,
}

impl specs::Component for Controllable {
    type Storage = specs::HashMapStorage<Control>;
}