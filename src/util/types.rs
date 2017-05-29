use gfx;
use gfx_device_gl;

pub type DeltaTime = f32;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub type FactoryType = gfx_device_gl::Factory;

pub type RenderTargetView = gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>;
pub type DepthStencilView = gfx::handle::DepthStencilView<gfx_device_gl::Resources, (gfx::format::D24_S8, gfx::format::Unorm)>;