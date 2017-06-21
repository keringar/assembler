use cgmath;
use resources::events;
use components;
use gfx::{self, Factory};
use gfx::traits::FactoryExt;
use gfx_device_gl;
use glutin;
use gfx_window_glutin;
use specs::{Fetch, Join, ReadStorage, System};

type ColorFormat = gfx::format::Rgba8;
type DepthFormat = gfx::format::DepthStencil;

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0], tex_coord: [ -0.5, -0.5 ] },
    Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0], tex_coord: [ -0.5, -0.5 ] },
    Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0], tex_coord: [ -0.5, -0.5 ] }
];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

    constant Uniforms {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        uniforms: gfx::ConstantBuffer<Uniforms> = "Uniforms",
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",
        color_buffer: gfx::RenderTarget<ColorFormat> = "o_Color",
        depth_buffer: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

pub struct WindowTarget<R: gfx::Resources> {
    pub color_buffer: gfx::handle::RenderTargetView<R, ColorFormat>,
    pub depth_buffer: gfx::handle::DepthStencilView<R, DepthFormat>,
    pub aspect_ratio: f32,
}

pub struct RenderSys<R, C, F, D> where R: gfx::Resources, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>, D: gfx::Device {
    window_target: WindowTarget<R>,
    encoder: gfx::Encoder<R, C>,
    factory: F,
    device: D,
}

pub fn new_gl_renderer(window: &glutin::Window) -> RenderSys<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer, gfx_device_gl::Factory, gfx_device_gl::Device> {
        let (device, mut factory, main_color, main_depth) = gfx_window_glutin::init_existing::<ColorFormat, DepthFormat>(window);
        let window_dim = window.get_inner_size_pixels().unwrap();
        let aspect_ratio = window_dim.0 as f32 / window_dim.1 as f32;

        let window_target = WindowTarget {
            color_buffer: main_color,
            depth_buffer: main_depth,
            aspect_ratio,
        };

        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        let pso = factory.create_pipeline_simple(include_bytes!("../shaders/default.vert"), include_bytes!("../shaders/default.frag"), pipe::new()).unwrap();

        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

        let texels = [[0x20, 0xA0, 0xC0, 0x00]];
        let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(gfx::texture::Kind::D2(1, 1, gfx::texture::AaMode::Single), &[&texels]).unwrap();

        let sinfo = gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Bilinear, gfx::texture::WrapMode::Clamp);

        let mut data = pipe::Data {
            vbuf: vertex_buffer,
            uniforms: factory.create_constant_buffer(1),
            color: (texture_view, factory.create_sampler(sinfo)),
            color_buffer: window_target.color_buffer.clone(),
            depth_buffer: window_target.depth_buffer.clone(),
        };
    
        RenderSys {
            window_target,
            encoder,
            factory,
            device,
        }
    }

impl<'a, R, C, F, D> System<'a> for RenderSys<R, C, F, D> where R: gfx::Resources, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>, D: gfx::Device<Resources = R, CommandBuffer = C> {
    type SystemData = (Fetch<'a, events::Events>, ReadStorage<'a, components::Render>, ReadStorage<'a, components::Transform>, ReadStorage<'a, components::Camera>);

    fn run(&mut self, data: Self::SystemData)  {
        let (events, render, transform, camera) = data;

        // Check if window resized, returning early upon finding a resize event
        for event in events.iter() {
            match *event {
                events::EventTypes::WindowResize(x, y) => {
                    self.window_target.aspect_ratio = x as f32 / y as f32;
                    break;
                }
                // TODO: Add in FOV change event
                _ => (),
            }
        }

        self.device.cleanup();
        self.encoder.clear(&self.window_target.color_buffer, [0.0, 0.0, 0.0, 1.0]);

        for (camera, camera_transform) in (&camera, &transform).join() {
            // Create view matrix

            // Take view matrix and projection matrix and create Uniforms struct with it

            for (render, transform) in (&render, &transform).join() {
                // Create model matrix

                // Update data using gfx::update_constant_buffer
                
                // Finally draw
                //self.encoder.draw(&slice, &self.pso, &self.pipeline_data)
            }
        }
        
        self.encoder.flush(&mut self.device);
    }
}