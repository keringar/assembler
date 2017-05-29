use resources;
use specs;
use util::types;
use world;

pub struct System {

}

impl System {
    pub fn new() -> System {
        System{}
    }
}

impl specs::System<types::DeltaTime> for System {
    fn run(&mut self, arg: specs::RunArg, time: types::DeltaTime) {
        use specs::Join;

        let logic = arg.fetch(|w| (w.read::<world::Control>()));
    }
}