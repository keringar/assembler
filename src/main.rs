extern crate bus;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate specs;

mod event;
mod system;
mod window;
mod world;
mod util;

fn main() {
    let root_logger = util::logger::create_logger();

    info!(root_logger, "Creating event manager");
    let mut events = event::EventManager::new();

    info!(root_logger, "Creating window");
    let (mut window, mut factory) = window::WindowBuilder::new()
        .with_title("Assembler")
        .build(&events);

    info!(root_logger, "Creating encoder channels");
    let (device_chan, render_chan) = util::mpsc_duplex::DuplexChannel::new();

    for _ in 0..2 {
        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
        device_chan
            .send(encoder)
            .expect("Unable to send encoder to game channel");
    }

    // game::new(game_chan, event_recv, factory, render_target)

    'main: loop {
        // Replace with check for game.is_running()
        // Translates windows events and turns them into game events
        // which are then sent to the game thread
        events.poll_events();

        // Temporary because no render system currently exists
        let encoder = render_chan.recv().unwrap();
        render_chan.send(encoder).unwrap();

        // The following code implements double buffering of encoders.
        // Two encoders are created, one is sent to the main thread
        // while the other lives on the render thread.
        let mut encoder = device_chan
            .recv()
            .expect("Unable to receiver encoder from render system");

        encoder.flush(&mut window.device);
        window.swap_buffers();

        device_chan
            .send(encoder)
            .expect("Unable to send encoder back to render system");
    }
}
