use engine::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug)]
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

    pub fn up_to(&self, max_len: usize) -> PrettyString {
        let mut output = PrettyString::new();
        let mut total_len = 0;
        for segment in &self.0 {
            let len = segment.0.len();
            if total_len + len <= max_len {
                output = output.add(segment.clone());
                total_len += len;
                if total_len == max_len {
                    return output;
                }
            } else {
                let substr = segment.0[..max_len - total_len].to_owned();
                return output.add((substr, segment.1.clone()));
            }
        }
        return output;
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
