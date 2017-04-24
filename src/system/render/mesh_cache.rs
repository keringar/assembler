use gfx;
use std::collections::HashMap;
use super::Mesh;

// Handle for indexing into MeshCache
pub struct MeshHandle {
    // Index refers to a bundle
    index: usize,
}

impl DrawHandle {
    fn new(index: usize) -> MeshHandle {
        MeshHandle {
            index: index,
        }
    }
}

pub struct BundleCache {
    cache: HashMap<MeshHandle, Mesh>,
}

impl 