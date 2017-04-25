use gfx;
use std::collections::HashMap;

use render::Mesh;

// Handle for indexing into MeshCache
pub struct MeshHandle {
    // Index refers to a bundle
    index: usize,
}

impl MeshHandle {
    fn new(index: usize) -> MeshHandle {
        MeshHandle {
            index: index,
        }
    }
}

pub struct MeshCache {
    cache: HashMap<MeshHandle, Mesh>,
}

impl MeshCache {
    pub fn add_mesh(mesh: Mesh) -> MeshHandle {
        unimplemented!()
    }

    pub fn get_slice(handle: &MeshHandle) -> Option<Mesh> {
        unimplemented!()
    }
}