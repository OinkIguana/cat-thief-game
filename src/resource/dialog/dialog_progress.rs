#[derive(Clone, PartialEq, Debug)]
pub struct DialogProgress(Option<f32>);

impl Default for DialogProgress {
    fn default() -> Self {
        DialogProgress(Some(1f32))
    }
}

impl DialogProgress {
    pub fn reset(&mut self) {
        self.0 = Some(1f32);
    }

    pub fn skip(&mut self) {
        self.0 = None;
    }

    pub fn progress(&mut self, amt: f32, limit: usize) {
        if let Some(prev) = self.0 {
            if (prev + amt) as usize >= limit {
                self.0 = None;
            } else {
                self.0 = Some(prev + amt);
            }
        }
    }

    pub fn current(&self) -> Option<usize> {
        self.0.map(|amt| amt as usize)
    }
}
