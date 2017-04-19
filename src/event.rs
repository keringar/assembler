use std::sync::mpsc;
use glutin;
use sys;

pub struct SenderHub {
    pub control: mpsc::Receiver
}