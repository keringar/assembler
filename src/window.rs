use gfx_device_gl;
use gfx_window_glutin;
use gfx::Device;
use glutin;

use util::types::{ColorFormat, DepthFormat};

pub struct WindowBuilder {
    title: String,
}

// Controls window and render device
// Sources events and sinks draw commands
pub struct Window {
    window_handle: glutin::Window,
    pub device: gfx_device_gl::Device,
}

impl Window {
    pub fn swap_buffers(&mut self) {
        self.window_handle
            .swap_buffers()
            .expect("Unable to swap window buffers");
        self.device.cleanup();
    }
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        let title = "Default title".to_string();

        WindowBuilder { title }
    }

    pub fn with_title(mut self, title: &str) -> WindowBuilder {
        self.title = title.to_string();
        self
    }

    pub fn build<'a>(self, event_loop: &'a glutin::EventsLoop) -> (Window, gfx_device_gl::Factory) {
        let window_builder = glutin::WindowBuilder::new().with_title(self.title);

        let (window_handle, device, factory, _, _) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, event_loop);

        let window = Window {
            window_handle,
            device,
        };

        (window, factory)
    }
}
