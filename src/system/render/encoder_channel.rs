use std::sync::mpsc;
use gfx;

pub struct EncoderChannel<R: gfx::Resources, C: gfx::CommandBuffer<R>> {
    pub receiver: mpsc::Receiver<gfx::Encoder<R, C>>,
    pub sender: mpsc::Sender<gfx::Encoder<R, C>>,
}