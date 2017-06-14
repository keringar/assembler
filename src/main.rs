extern crate cgmath;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate specs;

mod components;
mod events;
mod sys;

fn main() {
    let event_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().build(&event_loop).expect(
        "FATAL: Unable to create window",
    );

    let mut world = specs::World::new();

    world.register::<components::Control>();
    world.register::<components::Physics>();

    let player = world
        .create_entity()
        .with(components::Control { force: 1.0 })
        .with(components::Physics {
            velocity: cgmath::Vector2::new(0f32, 0f32),
            mass: 1.0,
        })
        .build();

    world.add_resource(events::Events::new());

    let mut dispatcher = specs::DispatcherBuilder::new()
        .add(sys::Control, "control", &[])
        .build();

    loop {
        // Event handling must be scoped due to lack of NLL
        {
            // Get a mutable reference to the events
            let mut event_mut = world.write_resource::<events::Events>();
            // Clear previous events
            event_mut.clear();
            // Handle window events and update keymap
            event_loop.poll_events(|event| event_mut.event_callback(event));
            // Flush keymap to events
            event_mut.flush();
        }

        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}
