use std::collections::VecDeque;
use model::message::Message;

#[derive(Clone, Default, Debug)]
pub struct DialogMessages {
    messages: VecDeque<Message>,
}

impl DialogMessages {
    pub fn add(&mut self, message: impl Into<Message>) {
        self.messages.push_back(message.into());
    }

    pub fn current(&self) -> Option<&Message> {
        self.messages.front()
    }

    pub fn dismiss(&mut self) {
        self.messages.pop_front();
    }
}
