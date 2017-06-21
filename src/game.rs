use components;
use resources::{Events, Time};
use specs;
use sys;
use window::Window;

pub struct Game<'a> {
    pub world: specs::World,
    dispatcher: specs::AsyncDispatcher<'a>,
    window: Window,
}

impl<'a> Game<'a> {
    pub fn new(window: Window) -> Self {
        let world = Game::init_world();

        let render_sys = sys::render::new_gl_renderer(&window);

        let mut dispatcher = specs::DispatcherBuilder::new()
            .add(sys::ControlSys, "player", &[])
            .add(sys::PhysicsSys, "physics", &["player"])
            .add_thread_local(render_sys)
            .build_async(world.res);

        Game {
            world,
            dispatcher: dispatcher,
            window,
        }
    }

    // Handles registration of all components and resources
    fn init_world() -> specs::World {
        let mut world = specs::World::new();

        world.register::<components::Camera>();
        world.register::<components::Player>();
        world.register::<components::Render>();
        world.register::<components::Physics>();
        world.register::<components::Transform>();

        world.add_resource(Events::new());
        world.add_resource(Time::new(60));

        world
    }

    // Handles fixed step updates
    pub fn update(&mut self) {
        self.dispatcher.wait_without_tl();
        self.dispatcher.dispatch();
        self.world.maintain();
    }

    // Called as many times as possible
    pub fn render(&mut self) {
        // Executes thread local systems after all async systems have finished
        self.dispatcher.wait();
        self.window.swap_buffers();
    }
}

