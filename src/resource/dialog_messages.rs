use std::collections::VecDeque;
use model::message::Message;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DialogProgress(Option<f32>);

impl Default for DialogProgress {
    fn default() -> Self {
        DialogProgress(Some(0f32))
    }
}

impl DialogProgress {
    pub fn reset(&mut self) {
        self.0 = Some(0f32);
    }

    pub fn skip(&mut self) {
        self.0 = None;
    }

    pub fn progress(&mut self, amt: f32, limit: usize) {
        if let Some(prev) = self.0 {
            if (prev + amt) as usize >= limit {
                self.0 = None;
            } else {
                self.0 = Some(prev + amt);
            }
        }
    }

    pub fn current(&self) -> Option<usize> {
        self.0.map(|amt| amt as usize)
    }
}

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

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}
