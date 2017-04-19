use std::sync::Arc;

use specs;
use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    Vertex {
        pos: [f32; 3] = "a_Pos",
    }

    constant Locals {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }

    pipeline Pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals"
        out_color: gfx::RenderTarget<DepthFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
};

const VERTEX_SHADER: &'static [u8] = b"
    #version 150 core

    in vec3 a_Pos;

    uniform Locals {
        mat4 u_Model;
        mat4 u_View;
        mat4 u_Proj;
    };

    out vec4 v_Color;

    void main() {
        gl_Position = u_Model * u_View * u_Proj * vec4(a_pos, 1.0);
        v_Color = vec4(a_Pos, 1.0);
    }
";

const FRAGMENT_SHADER: &'static [u8] = b"
    #version 150 core

    in vec4 v_Color;

    out vec4 Target0;
    
    void main() {
        Target0 = v_Color;
    }
";

#[derive(Clone)]
pub struct Drawable(usize, Locals);

impl specs::Component for Drawable {
    type Storage = specs::VecStorage<Drawable>;
}

pub struct Renderer<R: gfx::Resources> {
    out_color: gfx::handle::RenderTargetView<R, ColorFormat>,
    bundles: Arc<Vec<gfx::Bundle<R, pipe::Data<R>>>>,
}