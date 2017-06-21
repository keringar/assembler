extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate specs;

mod components;
mod game;
mod resources;
mod sys;
mod window;

fn main() {
    let window = window::Window::new();
    let mut game = game::Game::new(window);
}

