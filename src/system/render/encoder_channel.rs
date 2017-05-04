use std::sync::mpsc;
use gfx;

use gfx_device_gl;

pub struct EncoderChannel<R: gfx::Resources, C: gfx::CommandBuffer<R>> {
    pub sender: mpsc::Sender<gfx::Encoder<R, C>>,
    pub receiver: mpsc::Receiver<gfx::Encoder<R, C>>,
}

impl<R: gfx::Resources, C: gfx::CommandBuffer<R>> EncoderChannel<R, C> {
    pub fn from_factory(factory: &mut gfx_device_gl::Factory,)
        -> (EncoderChannel<R, C>, EncoderChannel<R, C>) {
        let (game_send, device_recv) = mpsc::channel();
        let (device_send, game_recv) = mpsc::channel();

        for _ in 0..2 {
            let enc: gfx::Encoder<_, _> = factory.create_command_buffer().into();
            game_send.send(enc).unwrap();
        }

        let game_channel = EncoderChannel {
            sender: game_send,
            receiver: game_recv,
        };

        let device_channel = EncoderChannel {
            sender: device_send,
            receiver: device_recv,
        };

        (game_channel, device_channel)
    }
}
