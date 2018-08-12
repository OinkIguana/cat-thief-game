//! The state of the player's progress through the game

mod main;

pub use self::main::MainState;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Progress {
    Started,
    Continuing,
}

impl Default for Progress {
    fn default() -> Self {
        Progress::Started
    }
}

#[derive(Eq, PartialEq, Default, Debug)]
pub struct State {
    main: MainState,
    main_progress: Progress,
}

impl State {
    pub fn enter(&mut self, state: MainState) {
        self.main = state;
        self.main_progress = Progress::Started;
    }

    pub fn is(&self, state: MainState) -> bool {
        self.main == state
    }

    pub fn just_started(&self, state: MainState) -> bool {
        self.is(state) && self.main_progress == Progress::Started
    }

    pub fn process(&mut self) {
        self.main_progress = Progress::Continuing
    }
}
