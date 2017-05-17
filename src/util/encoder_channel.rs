use std::sync::mpsc::{Sender, SendError, Receiver, RecvError, TryRecvError, channel};
use util::types::EncoderType;

pub struct EncoderChannel {
    receiver: Receiver<EncoderType>,
    sender: Sender<EncoderType>,
}

impl EncoderChannel {
    pub fn new() -> (EncoderChannel, EncoderChannel) {
        let (left_tx, left_rx) = channel();
        let (right_tx, right_rx) = channel();

        let left = EncoderChannel {
            receiver: left_rx,
            sender: right_tx,
        };

        let right = EncoderChannel {
            receiver: right_rx,
            sender: left_tx,
        };

        (left, right)
    }

    pub fn send(&self, encoder: EncoderType) -> Result<(), SendError<EncoderType>> {
        self.sender.send(encoder)
    }

    pub fn recv(&self) -> Result<EncoderType, RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<EncoderType, TryRecvError> {
        self.receiver.try_recv()
    }
}
