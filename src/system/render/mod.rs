mod encoder_channel;
mod system;
mod mesh_cache;

use gfx;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub use self::encoder_channel::EncoderChannel;
pub use self::mesh_cache::{DrawHandle, BundleCache};