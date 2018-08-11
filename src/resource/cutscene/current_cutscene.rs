use game_engine::prelude::*;
use crate::model::cutscene::{Cutscene, Action};

#[derive(Default)]
pub struct CurrentCutscene(Option<Box<dyn Cutscene>>);

impl CurrentCutscene {
    pub fn start(&mut self, cutscene: Box<dyn Cutscene>) {
        self.0 = Some(cutscene);
    }

    pub fn is_over(&self) -> bool {
        self.0.as_ref().map(|cutscene| cutscene.is_over()).unwrap_or(true)
    }

    pub fn actions(&mut self, world: &World) -> Option<Vec<Action>> {
        let values = self.0.as_mut().map(|cutscene| (
            cutscene.actions(world),
            cutscene.is_over(),
        ));
        if let Some((actions, is_over)) = values {
            if is_over {
                self.0 = None;
            }
            Some(actions)
        } else {
            None
        }
    }
}
