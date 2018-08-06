use super::pretty_string::PrettyString;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Message {
    speaker: Option<String>,
    message: PrettyString,
}

impl Message {
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

impl From<String> for Message {
    fn from(string: String) -> Self {
        Self::from(string.as_str())
    }
}


impl From<&str> for Message {
    fn from(string: &str) -> Self {
        if let Some(index) = string.find(":") {
            Message {
                speaker: Some(string[..index].to_string()),
                message: PrettyString::parse(&string[index + 1..]),
            }
        } else {
            Message {
                speaker: None,
                message: PrettyString::parse(&string[..]),
            }
        }
    }
}
