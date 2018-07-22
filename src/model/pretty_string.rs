use engine::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Attribute {
    Font(&'static Font),
    Color(Color),
}

pub trait PrettyStringSegment {
    fn convert(self) -> (String, Vec<Attribute>);
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct PrettyString(pub Vec<(String, Vec<Attribute>)>);

impl PrettyString {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<Segment: PrettyStringSegment>(mut self, segment: Segment) -> Self {
        self.0.push(segment.convert());
        self
    }

    pub fn len(&self) -> usize {
        self.0
            .iter()
            .map(|(string, _)| string.len())
            .fold(0, |a, b| a + b)
    }
}

impl<'a> PrettyStringSegment for &'a str {
    fn convert(self) -> (String, Vec<Attribute>) {
        (self.to_owned(), vec![])
    }
}

impl PrettyStringSegment for String {
    fn convert(self) -> (String, Vec<Attribute>) {
        (self, vec![])
    }
}

impl PrettyStringSegment for (String, Vec<Attribute>) {
    fn convert(self) -> (String, Vec<Attribute>) {
        self
    }
}

impl<'a> PrettyStringSegment for (&'a str, Vec<Attribute>) {
    fn convert(self) -> (String, Vec<Attribute>) {
        (self.0.to_owned(), self.1)
    }
}

impl<'a> Into<PrettyString> for &'a str {
    fn into(self) -> PrettyString {
        PrettyString::new().add(self)
    }
}

impl<'a> Into<PrettyString> for (&'a str, Vec<Attribute>) {
    fn into(self) -> PrettyString {
        PrettyString::new().add(self)
    }
}

impl Into<PrettyString> for String {
    fn into(self) -> PrettyString {
        PrettyString::new().add(self)
    }
}

impl Into<PrettyString> for (String, Vec<Attribute>) {
    fn into(self) -> PrettyString {
        PrettyString::new().add(self)
    }
}
