pub mod ping;
use super::{Receiver, Sender, Task};

pub enum ChannelType {
    Task {
        sender: Option<Sender<Task>>,
        receiver: Option<Receiver<Task>>,
    },
    None,
}

pub trait TaskHandler {
    fn start(&mut self);
    fn exit(&mut self);
    fn get_channel(&mut self) -> ChannelType;
}
