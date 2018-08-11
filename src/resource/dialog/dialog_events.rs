#[derive(Clone, Default, Debug)]
pub struct DialogEvents(Vec<String>);

impl DialogEvents {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn add(&mut self, event: String) {
        self.0.push(event);
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }
}
