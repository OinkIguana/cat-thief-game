use super::pretty_string::PrettyString;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Message {
    speaker: Option<String>,
    message: PrettyString,
}

impl Message {
    pub fn new(speaker: String, message: impl Into<PrettyString>) -> Self {
        Message {
            speaker: Some(speaker),
            message: message.into(),
        }
    }

    pub fn anon(message: impl Into<PrettyString>) -> Self {
        Message {
            speaker: None,
            message: message.into(),
        }
    }

    pub fn speaker(&self) -> &Option<String> {
        &self.speaker
    }

    pub fn message(&self) -> &PrettyString {
        &self.message
    }

    pub fn len(&self) -> usize {
        self.message.len()
    }
}
