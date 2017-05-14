use std::sync::mpsc::{Sender, SendError, Receiver, RecvError, channel};

pub struct DuplexChannel<T> {
    receiver: Receiver<T>,
    sender: Sender<T>,
}

impl<T> DuplexChannel<T> {
    pub fn new() -> (DuplexChannel<T>, DuplexChannel<T>) {
        let (left_tx, left_rx) = channel();
        let (right_tx, right_rx) = channel();

        let left = DuplexChannel {
            receiver: left_rx,
            sender: right_tx,
        };

        let right = DuplexChannel {
            receiver: right_rx,
            sender: left_tx,
        };

        (left, right)
    }

    pub fn send(&self, data: T) -> Result<(), SendError<T>> {
        self.sender.send(data)
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }
}
