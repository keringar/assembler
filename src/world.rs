use cgmath::{Point3}
use specs;
use system::render;

// Defines Transformation component
#[derive(Clone)]
pub struct Transform {
    pub pos: Point3<f32>,
    pub scale: f32,
}

impl specs::Component for Transform {
    type Storage = specs::VecStorage<Transform>;
}

// Defines drawable component, all renderable entities must have this
pub struct Drawable {
    slice: render::DrawHandle,
    pipeline: pso,
    data: data
}