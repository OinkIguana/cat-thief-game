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
    state::{State, MainState},
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
    StateChange(MainState),
}

pub trait Cutscene: Sync + Send {
    fn is_over(&self) -> bool;
    fn run(&mut self, world: &mut World);
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

    fn run(&mut self, world: &mut World) {
        if self.delay != 0 {
            self.delay -= 1;
            return;
        }
        'outer: loop {
            match self.steps.first() {
                None => break,
                Some(Step::StateChange(new_state)) => {
                    self.steps = &self.steps[1..];
                    world.write_resource::<State>().enter(*new_state);
                }
                Some(Step::Break) => {
                    self.steps = &self.steps[1..];
                    break;
                }
                Some(Step::Delay(timer)) => {
                    self.delay = *timer;
                    break
                }
                Some(Step::Move(target, path)) => {
                    self.steps = &self.steps[1..];
                    let entities = world.entities();
                    let id = world.read_storage::<Id>();
                    for (entity, id) in (&*entities, &id).join() {
                        if id == target {
                            world.write_storage::<MovePath>().insert(entity, MovePath::from(*path)).unwrap();
                        }
                    }
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
                    world.write_resource::<DialogMessages>().start(story())
                }
            }
        }
    }
}
