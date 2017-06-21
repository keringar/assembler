use cgmath::{Quaternion, Rad, Vector3, Zero};
use specs::{Component, HashMapStorage, VecStorage};

// Render system will render a scene from the perspective of camera entities
#[derive(Clone, Debug)]
pub struct Camera {
    pub zoom: f32,
    pub up_dir: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            zoom: 1.0,
            up_dir: Vector3::unit_y(),
        }
    }
}

impl Component for Camera {
    type Storage = HashMapStorage<Camera>;
}

// Allows entities with this component to be moved around by the player
#[derive(Clone, Debug)]
pub struct Player {
    pub force: f32,
}

impl Player {
    pub fn new(force: f32) -> Player {
        Player { force }
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Player>;
}

// Allows the render system to render entities
#[derive(Clone, Debug)]
pub struct Render {
    // index into some texture resource
    pub material: usize,
}

impl Component for Render {
    type Storage = VecStorage<Render>;
}

// Handles physics for an entity, inertia, angular momentum, etc.
#[derive(Clone, Debug)]
pub struct Physics {
    pub velocity: Vector3<f32>,
    pub mass: f32,
}

impl Physics {
    pub fn new(mass: f32) -> Physics {
        Physics {
            velocity: Vector3::zero(),
            mass,
        }
    }

    pub fn apply_force(&mut self, direction: Vector3<f32>, magnitude: f32, dt: f32) {
        let acceleration = magnitude / self.mass;
        self.velocity += direction * acceleration * dt;
    }
}

impl Component for Physics {
    type Storage = VecStorage<Physics>;
}

// Gives entities a position, rotation and scale
#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub orientation: Rad<f32>,
    pub scale: f32,
}

impl Component for Transform {
    type Storage = VecStorage<Transform>;
}

