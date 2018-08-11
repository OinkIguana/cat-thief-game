//! Input events mapped through the `ControlScheme` for uniform processing.

use game_engine::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ControlEvent {
    Left(u8),
    Right(u8),
    Up(u8),
    Down(u8),
    Action(Option<Point>),
    Cancel(Option<Point>),
    Menu(Option<Point>),
    Run(Option<Point>),
}

#[derive(Clone, Default, Debug)]
pub struct ControlEvents(Vec<ControlEvent>);

impl ControlEvents {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn add(&mut self, event: ControlEvent) {
        self.0.push(event);
    }

    pub fn iter(&self) -> impl Iterator<Item = &ControlEvent> {
        self.0.iter()
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct ControlState {
    pub axis_h: i8,
    pub axis_v: i8,
    pub action: bool,
    pub cancel: bool,
    pub menu: bool,
    pub run: bool,
}
