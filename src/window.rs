use gfx_device_gl;
use gfx_window_glutin;
use glutin;
use std::ops::Deref;
use gfx::Device;

use types::{ColorFormat, DepthFormat};

pub struct WindowBuilder {
    title: String,
}

// Controls window and render device
// Sources events and sinks draw commands
pub struct Window {
    window_handle: glutin::Window,
    event_loop: glutin::EventsLoop,
    pub device: gfx_device_gl::Device,
}

impl Window {
    pub fn swap_buffers() {
        self.window_handle
            .swap_buffers()
            .expect("Unable to swap window buffers");
        self.device.cleanup();
    }

    pub fn get_event_loop(&self) -> &glutin::EventsLoop {
        &self.event_loop
    }
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder { title: "Title not set".to_string() }
    }

    pub fn with_title(mut self, title: &str) -> WindowBuilder {
        self.title = title.to_string();
        self
    }

    pub fn build(self) -> (Window, gfx_device_gl::Factory) {
        let window_builder = glutin::WindowBuilder::new().with_title(self.title);

        let (window_handle, device, factory, _, _) = gfx_window_glutin::init::<ColorFormat,
                                                                               DepthFormat>(window_builder);

        let window = Window {
            window_handle,
            device,
        };

        (window, factory)
    }
}
