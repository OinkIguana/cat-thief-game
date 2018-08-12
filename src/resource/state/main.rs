#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum MainState {
    Start,
    RunToTheAlley,
    ArriveInTheAlley,
    HideSomewhere,
    End,
}

impl Default for MainState {
    fn default() -> Self {
        MainState::Start
    }
}
