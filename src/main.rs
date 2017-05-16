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
mod game;
mod system;
mod window;
mod world;
mod util;

use util::logger::{self, LogType};
use util::types::{ColorFormat, DepthFormat};

fn main() {
    let root_logger = logger::create_logger(LogType::Terminal);

    info!(root_logger, "Creating event manager");
    // render_receiver is only created so that when the window is created,
    // it can immediately send over it's dimensions
    let (mut event_source, render_receiver) = event::EventManager::new();

    info!(root_logger, "Creating window");
    let mut window = window::WindowBuilder::new()
        .with_title("Assembler")
        .build(&mut event_source);

    let (_, mut factory, rtv, dsv) = gfx_window_glutin::init_existing::<ColorFormat, DepthFormat>(&window,);

    info!(root_logger, "Creating encoder channels");
    let (device_chan, render_chan) = util::EncoderChannel::new();

    // Double buffer encoders
    for _ in 0..2 {
        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
        device_chan
            .send(encoder)
            .expect("Unable to send encoder to game channel");
    }

    let mut game = game::Game::new(
        &mut event_source,
        render_receiver,
        render_chan,
        &mut factory,
    );

    while game.is_running() {
        // Translates windows events and turns them into game events
        // which are then sent to all listeners
        event_source.poll_events();

        game.update();

        // The following code implements double buffering of encoders.
        // Two encoders are created, one is sent to the main thread
        // while the other lives on the render thread.
        let mut encoder = device_chan
            .recv()
            .expect("Unable to receive encoder from render system");

        encoder.flush(&mut window.device);
        window.swap_buffers();

        device_chan
            .send(encoder)
            .expect("Unable to send encoder back to render system");
    }
}
