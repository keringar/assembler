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
mod game_manager;
mod system;
mod resources;
mod world;
mod util;

use gfx::Device;

use util::logger::{self, LogType};
use util::types::{ColorFormat, DepthFormat};

fn main() {
    let root_logger = logger::create_logger(LogType::Terminal);

    info!(root_logger, "Creating event manager");
    // render_events is only created so that when the window is created,
    // it can immediately send over its dimensions to the render threads
    let (mut event_manager, render_events) = event::EventManager::new();

    info!(root_logger, "Creating window");
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Assembler")
        .with_vsync();

    let (window, mut device, mut factory, rtv, dsv) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, &event_manager);

    info!(root_logger, "Creating encoder channels");
    let (device_chan, render_chan) = util::DuplexChannel::new();

    // Double buffer encoders
    for _ in 0..2 {
        device_chan
            .send(factory.create_command_buffer().into())
            .expect("Unable to send encoder to game channel");
    }

    info!(root_logger, "Creating resource manager");
    let resources = resources::Manager::new(factory);

    info!(root_logger, "Creating game manager");
    // Handles the ECS as well as any resource loading. Calls into
    // logic module to handle actual game logic
    let mut manager = game_manager::Manager::new(
        &mut event_manager,
        render_events,
        render_chan,
        resources,
        rtv,
        dsv,
    );

    while manager.is_running() {
        // Translates windows events and turns them into game events
        // which are then sent to all listeners
        event_manager.poll_events();

        manager.update();

        // The following code implements double buffering of encoders.
        // Two encoders are created, one is sent to the main thread
        // while the other lives on the render threads.
        match device_chan.recv() {
            Ok(mut encoder) => {
                encoder.flush(&mut device);
                window.swap_buffers().expect("Unable to swap window buffers");
                device.cleanup();
                device_chan
                    .send(encoder)
                    .expect("Unable to send encoder back to render system");
            }
            Err(_) => panic!("Unable to receive encoder from render system"),
        }
    }
}
