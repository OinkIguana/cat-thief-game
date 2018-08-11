use crate::model::cutscene::Action;

#[derive(Default, Debug)]
pub struct CutsceneActions(Vec<Action>);

impl CutsceneActions {
    pub fn clear(&mut self) {
        self.0 = vec![];
    }

    pub fn set(&mut self, action: Vec<Action>) {
        self.0 = action;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Action> {
        self.0.iter()
    }
}
