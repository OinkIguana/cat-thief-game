use crate::model::cutscene::Cutscene;

#[derive(Default)]
pub struct CurrentCutscene(Option<Box<dyn Cutscene>>);

impl CurrentCutscene {
    pub fn start(&mut self, cutscene: Box<dyn Cutscene>) {
        if !cutscene.is_over() {
            self.0 = Some(cutscene);
        }
    }

    pub fn is_over(&self) -> bool {
        self.0.as_ref().map(|cutscene| cutscene.is_over()).unwrap_or(true)
    }

    pub fn take(&mut self) -> Option<Box<dyn Cutscene>> {
        self.0.take()
    }
}
