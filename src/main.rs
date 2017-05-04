#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate specs;
extern crate cgmath;

mod event;
mod logger;
mod system;
mod types;
mod window;
mod world;

fn main() {
    let root_logger = logger::create_logger();

    info!(root_logger, "Creating window");
    let (mut window, mut factory) = window::WindowBuilder::new()
        .with_title("Assembler")
        .build();

    info!(root_logger, "Creating event manager");
    let (event_send, event_recv) = event::SenderHub::new();

    info!(root_logger, "Creating encoder channels");
    let (game_channel, device_channel) = system::render::EncoderChannel::from_factory(&mut factory);

    // game::new(game_channel, event_recv, factory, render_target)

    'main: loop {
        let mut encoder = match device_channel.receiver.recv() {
            Ok(encoder) => encoder,
            Err(_) => break 'main,
        };

        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => event_send.process_glutin(event),
            }
        }

        encoder.flush(&mut window.device);
        device_channel
            .sender
            .send(encoder)
            .expect("Unable to send encoder back to game thread");
        window.swap_buffers();
    }
}
