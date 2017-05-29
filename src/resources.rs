// Resource manager
use gfx;
use util::types;

pub struct Manager {
    factory: types::FactoryType,
}

impl Manager {
    pub fn new(factory: types::FactoryType) -> Manager {
        Manager {
            factory
        }
    }

    pub fn load(&mut self) {
        self.load_textures();

        // Load shaders
        // Load graphics pipelines
    }

    fn load_textures(&mut self) {

    }
}