use cgmath;
use event::{Event, EventReceiver, EventManager};
use gfx;
use specs;
use system as sys;
use std;
use resources;
use world;
use util::{DuplexChannel, types};

pub struct Manager {
    events: EventReceiver,
    //player: world::Control,
    planner: specs::Planner<types::DeltaTime>,
    is_running: bool,
    last_time: std::time::Instant,
}

impl Manager {
    pub fn new<R: gfx::Resources, C: 'static + Send>(
        event_manager: &mut EventManager,
        render_event: EventReceiver,
        encoder_chan: DuplexChannel<gfx::Encoder<R, C>>,
        resources: resources::Manager,
        rtv: types::RenderTargetView,
        dsv: types::DepthStencilView,
    ) -> Manager
    {
        let mut ecs_planner = create_ecs();

        // Player events
        let control_system = sys::control::System::new(event_manager.add_receiver());
        // Game logic
        //let logic_system = sys::logic::System::new(&mut resource_manager);
        //Render everything
        let render_system = sys::render::System::new(render_event, encoder_chan, rtv, dsv, &resources);

        ecs_planner.add_system(control_system, "Controls", 1);
        //ecs_planner.add_system(logic_system, "Logic", 2);
        ecs_planner.add_system(render_system, "Render", 3);

        Manager {
            events: event_manager.add_receiver(),
            planner: ecs_planner,
            is_running: true,
            last_time: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self) {
        loop {
            match self.events.try_recv() {
                Ok(event) => {
                    match event {
                        Event::Quit => self.is_running = false,
                        _ => (),
                    }
                }
                Err(_) => break,
            }
        }

        let elapsed = self.last_time.elapsed();
        self.last_time = std::time::Instant::now();
        let dt = elapsed.subsec_nanos() as f32 / 1e9 + elapsed.as_secs() as f32;

        self.planner.dispatch(dt);
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

fn create_ecs() -> specs::Planner<types::DeltaTime> {
    let mut world = specs::World::new();
    world.register::<world::Transform>();
    world.register::<world::Inertial>();
    world.register::<world::Control>();
    world.register::<world::Renderable>();
    world.register::<world::Chunk>();

    world.create().with(world::Renderable{}).build();

    specs::Planner::new(world)
}