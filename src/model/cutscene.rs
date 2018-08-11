use game_engine::prelude::*;
use serde::Deserialize;
use ron::de::from_str;
use inkgen::runtime::Story;

use crate::component::{
    id::Id,
    behavior::MovePath,
};
use crate::resource::{
    dialog::{DialogEvents, DialogMessages},
};

#[derive(Clone, Debug)]
pub enum Step<E>
where for<'a> E: Deserialize<'a> + PartialEq + Sync + Send + 'static {
    Move(Id, &'static [Point<f32>]),
    AwaitMoveEnd(Id),
    AwaitDialogEvent(E),
    AwaitDialogEnd,
    StartDialog(fn() -> Story),
    Delay(u32),
    Break,
}

#[derive(Clone, Debug)]
pub enum Action {
    Move(Id, &'static [Point<f32>]),
    Dialog(fn() -> Story),
}

pub trait Cutscene: Sync + Send {
    fn is_over(&self) -> bool;
    fn actions(&mut self, world: &World) -> Vec<Action>;
}

pub struct StandardCutscene<E>
where for<'a> E: Deserialize<'a> + PartialEq + Sync + Send + 'static {
    steps: &'static [Step<E>],
    delay: u32,
}

impl<E> StandardCutscene<E>
where for<'a> E: Deserialize<'a> + PartialEq + Sync + Send + 'static {
    pub const fn new(steps: &'static [Step<E>]) -> Self {
        Self {
            steps,
            delay: 0,
        }
    }

    pub fn boxed(steps: &'static [Step<E>]) -> Box<Self> {
        Box::new(Self::new(steps))
    }
}

impl<E> Cutscene for StandardCutscene<E>
where for<'a> E: Deserialize<'a> + PartialEq + Sync + Send + 'static {
    fn is_over(&self) -> bool {
        self.steps.is_empty() && self.delay == 0
    }

    fn actions(&mut self, world: &World) -> Vec<Action> {
        let mut actions = vec![];
        if self.delay != 0 {
            self.delay -= 1;
            return actions;
        }
        'outer: loop {
            match self.steps.first() {
                None => break,
                Some(Step::Break) => {
                    self.steps = &self.steps[1..];
                    break;
                }
                Some(Step::Delay(timer)) => {
                    self.delay = *timer;
                    break
                }
                Some(Step::Move(id, move_path)) => {
                    self.steps = &self.steps[1..];
                    actions.push(Action::Move(*id, *move_path));
                }
                Some(Step::AwaitMoveEnd(target)) => {
                    let (entities, id) = (
                        world.entities(),
                        world.read_storage::<Id>(),
                    );
                    for (entity, id) in (&*entities, &id).join() {
                        if id == target {
                            if world.read_storage::<MovePath>().get(entity).map(MovePath::is_empty).unwrap_or(true) {
                                break;
                            } else {
                                break 'outer;
                            }
                        }
                    }
                    self.steps = &self.steps[1..];
                }
                Some(Step::AwaitDialogEvent(target)) => {
                    let emitted = world.read_resource::<DialogEvents>()
                        .iter()
                        .map(|string| from_str(string).unwrap())
                        .collect::<Vec<_>>()
                        .contains(target);
                    if emitted {
                        self.steps = &self.steps[1..];
                    } else {
                        break
                    }
                }
                Some(Step::AwaitDialogEnd) => {
                    if world.read_resource::<DialogMessages>().is_empty() {
                        self.steps = &self.steps[1..];
                    } else {
                        break
                    }
                }
                Some(Step::StartDialog(story)) => {
                    self.steps = &self.steps[1..];
                    actions.push(Action::Dialog(*story));
                }
            }
        }
        actions
    }
}
