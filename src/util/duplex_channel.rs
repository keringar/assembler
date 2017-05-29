use std::sync::mpsc::{Sender, SendError, Receiver, RecvError, TryRecvError, channel};

pub struct DuplexChannel<T>{
    receiver: Receiver<T>,
    sender: Sender<T>,
}

impl<T> DuplexChannel<T> {
    pub fn new() -> (DuplexChannel<T>, DuplexChannel<T>) {
        let (left_tx, left_rx) = channel();
        let (right_tx, right_rx) = channel();

        let right = DuplexChannel {
            receiver: left_rx,
            sender: right_tx,
        };

        let left = DuplexChannel {
            receiver: right_rx,
            sender: left_tx,
        };

        (left, right)
    }

    pub fn send(&self, data: T) -> Result<(), SendError<T>> {
        self.sender.send(data)
    }

    #[allow(dead_code)]
    pub fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }

    #[allow(dead_code)]
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }
}
