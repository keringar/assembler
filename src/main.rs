#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate specs;
extern crate cgmath;
extern crate rand;

mod event;
mod game;
mod sys;
mod world;

fn main() {
    let root_logger = create_logger();
    
    // let (ev_send, ev_recv) = event::SenderHub::new();

    let window_builder = glutin::WindowBuilder::new()
        .with_title("Assembler".to_string());
    
    info!("Creating window");
    let (window, device, mut factory, main_color, main_depth) = 
        gfx_window_glutin::init::<sys::render::ColorFormat, sys::render::DepthFormat>(window_builder);
}

fn create_logger() -> slog::Logger {
    use slog::*;
    use std::sync::Arc;
    use std::fs::File;

    // Create terminal drain
    let term_decorator = slog_term::TermDecorator::new().build();
    let term_drain = slog_term::CompactFormat::new(term_decorator)
        .build()
        .fuse();
    let async_term_drain = slog_async::Async::new(term_drain).build().fuse();

    // Try to create file drain
    let async_file_drain = match File::create("app.log") {
        Ok(file) => {
            let file_decorator = slog_term::PlainDecorator::new(file);
            let file_drain = slog_term::FullFormat::new(file_decorator).build().fuse();
            let async_file_drain = slog_async::Async::new(file_drain).build().fuse();
            Some(async_file_drain)
        },
        Err(_) => None,
    };

    // Only combine file and console drains if file drain was created, otherwise, just use terminal
    return match async_file_drain {
        Some(async_file_drain) => { 
            let combined_drain = slog::Duplicate::new(async_file_drain, async_term_drain).fuse();
            slog::Logger::root(Arc::new(combined_drain), o!("version" => env!("CARGO_PKG_VERSION")))
        },
        None => {
            let logger = slog::Logger::root(Arc::new(async_term_drain), o!("version" => env!("CARGO_PKG_VERSION")));
            warn!(logger, "Could not open log file, logging to terminal only");
            logger
        }
    };
}