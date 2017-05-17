use cgmath;
use event::{Event, EventReceiver, EventManager};
use gfx;
use specs;
use system;
use std;
use world;
use util::{EncoderChannel, types};

pub struct Game {
    events: EventReceiver,
    //player: world::Control,
    planner: specs::Planner<types::DeltaTime>,
    is_running: bool,
    last_time: std::time::Instant,
}

impl Game {
    pub fn new<R, F>(
        manager: &mut EventManager,
        render_event: EventReceiver,
        encoder_chan: EncoderChannel,
        factory: &mut F,
    ) -> Game
    where
        R: 'static + gfx::Resources,
        F: gfx::Factory<R>,
    {

        let game_receiver = manager.add_receiver();
        let control_receiver = manager.add_receiver();

        use cgmath::Zero;
        let player_intertial = world::Inertial { velocity: cgmath::Vector3::zero() };

        let player_control = world::Control { acceleration: 0f32 };

        let mut planner = {
            let mut world = specs::World::new();
            world.register::<world::Transform>();
            world.register::<world::Inertial>();
            world.register::<world::Control>();

            world
                .create_now()
                .with(player_control)
                .with(player_intertial)
                .build();

            specs::Planner::new(world)
        };

        planner.add_system(system::control::System::new(control_receiver), "Control", 1);
        planner.add_system(
            system::render::System::new(render_event, encoder_chan),
            "Render",
            0,
        );

        Game {
            events: game_receiver,
            planner,
            is_running: true,
            last_time: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self) {
        match self.events.try_recv() {
            Ok(event) => {
                match event {
                    Event::Quit => self.is_running = false,
                    _ => (),
                }
            }
            Err(_) => (),
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
