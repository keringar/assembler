use gfx;
use event::{EventReceiver, Event};
use specs;
use std::vec::Vec;
use util::{DuplexChannel, types};
use world;
use resources;

pub struct System<R: gfx::Resources, C> {
    dimensions: (u32, u32),
    event_receiver: EventReceiver,
    render_chan: DuplexChannel<gfx::Encoder<R, C>>,
}

impl<R, C> System<R, C> where R: gfx::Resources {
    pub fn new(event_receiver: EventReceiver, render_chan: DuplexChannel<gfx::Encoder<R, C>>, render_view: types::RenderTargetView, depth_view: types::DepthStencilView, resource_manager: &resources::Manager) -> System<R, C> {
        System {
            dimensions: (0, 0),
            event_receiver,
            render_chan,
        }
    }

    fn update_dimensions(&mut self) {
        loop {
            match self.event_receiver.try_recv() {
                Ok(event) => {
                    match event {
                        Event::Resize(x, y) => {
                            self.dimensions = (x, y);
                        },
                        _ => (),
                    }
                }
                Err(_) => break,
            }
        }
    }
}

impl<R, C> specs::System<types::DeltaTime> for System<R, C> where R: gfx::Resources, C: Send {
    fn run(&mut self, arg: specs::RunArg, time: types::DeltaTime) {
        use specs::Join;

        self.update_dimensions();

        let renderable = arg.fetch(|w| (w.read::<world::Renderable>()));

        let mut encoder = self.render_chan
            .recv()
            .expect("Unable to receive encoder from main thread");
        
        let renderable: Vec<&world::Renderable> = renderable.join().collect();

        for render in renderable {

        }

        self.render_chan
            .send(encoder)
            .expect("Unable to send encoder back to main thread");
    }
}

// GFX STUFF
gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<types::ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] }
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
