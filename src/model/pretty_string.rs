use std::collections::HashMap;
use game_engine::prelude::*;
use lazy_static::lazy_static;
use crate::font::default;

lazy_static! {
    static ref RULES: HashMap<&'static str, Attributes> = {
        let mut map = HashMap::new();
        map.insert("location", Attributes { font: None, color: Some(Color::BLUE) });
        map.insert("thought", Attributes { font: Some(&default::ITALIC_20), color: Some(0x555050ff.into()) });
        map.insert("yell", Attributes { font: Some(&default::ITALIC_20), color: None });
        map
    };
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Attributes {
    pub font: Option<&'static Font>,
    pub color: Option<Color>,
}

impl Attributes {
    pub fn override_with(self, other: &Attributes) -> Attributes {
        Attributes {
            font: other.font.or(self.font),
            color: other.color.or(self.color),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct PrettyString(pub Vec<(String, Attributes)>);

impl PrettyString {
    pub fn parse(string: &str) -> Self{
        parser::parse(string)
    }

    pub fn len(&self) -> usize {
        self.0
            .iter()
            .map(|(string, _)| string.len())
            .fold(0, |a, b| a + b)
    }

    pub fn plain(&self) -> String {
        self.0.iter()
            .map(|(string, _)| string.clone())
            .collect()
    }
}

mod parser {
    use super::{PrettyString, Attributes, RULES};
    use std::ops::{Generator, GeneratorState};

    enum State {
        Start,
        Open(String),
        Close,
    }

    pub enum Token {
        StartSegment(String),
        EndSegment,
        Text(String),
    }

    fn lmm(text: &str, state: State) -> (Token, &str) {
        use self::State::*;
        let mut chars = text.chars();
        match (state, chars.next()) {
            (Start, Some('<')) => lmm(chars.as_str(), Open("".to_owned())),
            (Start, Some('>')) => lmm(chars.as_str(), Close),
            (Start, Some(ch)) => (Token::Text(ch.to_string()), chars.as_str()),
            (Open(ref st), Some('<')) if st.is_empty() => (Token::Text("<".to_owned()), text),
            (Open(ref st), Some('>')) if st.is_empty() => (Token::Text(">".to_owned()), text),
            (Open(ref st), Some(':')) if st.is_empty() => panic!("Missing rule name in dialog string"),
            (Open(st), Some(':')) => (Token::StartSegment(st), chars.as_str()),
            (Open(st), Some(ch)) => lmm(chars.as_str(), Open(st + &ch.to_string())),
            (Close, _) => (Token::EndSegment, text),
            _ => panic!("Unexpected end of string"),
        }
    }

    fn tokenize(mut string: &'a str) -> impl Iterator<Item = Token> + 'a {
        let gen = move || {
            while !string.is_empty() {
                let (token, rest) = lmm(string, State::Start);
                yield token;
                string = rest;
            }
        };
        generator_to_iterator(gen)
    }

    fn resolve_rules(rules: &[String]) -> Attributes {
        rules.iter()
            .fold(
                Attributes::default(),
                |attrs, rule| attrs.override_with(
                    RULES
                        .get(rule.as_str())
                        .expect(&format!("Missing rule {} in dialog string", rule))
                )
            )
    }

    pub(super) fn parse(string: &str) -> PrettyString {
        let mut segments = vec![];
        let mut rules = vec![];
        let mut segment = String::new();
        for token in tokenize(string) {
            match token {
                Token::StartSegment(name) => {
                    if !segment.is_empty() {
                        segments.push((segment, resolve_rules(&rules)));
                        segment = String::new();
                    }
                    rules.push(name);
                }
                Token::EndSegment => {
                    if !segment.is_empty() {
                        segments.push((segment, resolve_rules(&rules)));
                        segment = String::new();
                    }
                    rules.pop().expect("Ran out of rules to pop");
                }
                Token::Text(text) => segment = segment + &text,
            }
        }
        if !segment.is_empty() {
            segments.push((segment, resolve_rules(&rules)));
        }
        PrettyString(segments)
    }

    fn generator_to_iterator<G>(g: G) -> impl Iterator<Item = G::Yield>
    where G: Generator<Return = ()> {
        struct It<G>(G);

        impl<G: Generator<Return = ()>> Iterator for It<G> {
            type Item = G::Yield;

            fn next(&mut self) -> Option<Self::Item> {
                unsafe {
                    match self.0.resume() {
                        GeneratorState::Yielded(y) => Some(y),
                        GeneratorState::Complete(()) => None,
                    }
                }
            }
        }

        It(g)
    }
}
