use std::ops::{Generator, GeneratorState};

enum State {
    Start,
    Open(String),
    DoubleOpen,
    Close,
    OpenClose,
}

pub enum Token {
    StartSegment(String),
    EndSegment,
    Text(String),
}

fn lmm(mut text: String, state: State) -> (Token, String) {
    let rest = text.split_off(1);
    use self::State::*;
    match (state, text.pop().unwrap()) {
        (Start, '{')                            => lmm(rest, Open("".to_owned())),
        (Start, '}')                            => lmm(rest, Close),
        (Start, ch)                             => (Token::Text(ch.to_string()), rest),
        (Open(ref st), '{') if st.is_empty()    => lmm(rest, DoubleOpen),
        (Open(ref st), '}') if st.is_empty()    => lmm(rest, OpenClose),
        (Open(ref st), ':') if st.is_empty()    => panic!("Missing rule name in dialog string"),
        (Open(st), ':')                         => (Token::StartSegment(st), rest),
        (Open(st), ch)                          => lmm(rest, Open(st + &ch.to_string())),
        (Close, ch)                             => (Token::EndSegment, ch.to_string() + &rest),
        (DoubleOpen, ch)                        => (Token::Text("{".to_owned()), ch.to_string() + &rest),
        (OpenClose, ch)                         => (Token::Text("}".to_owned()), ch.to_string() + &rest),
    }
}

pub fn tokenize(mut string: String) -> impl Iterator<Item = Token> {
    let gen = || {
        while !string.is_empty() {
            let (token, rest) = lmm(string, State::Start);
            yield token;
            string = rest;
        }
    };
    generator_to_iterator(gen)
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
