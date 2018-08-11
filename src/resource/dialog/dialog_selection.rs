#[derive(Clone, PartialEq, Default, Debug)]
pub struct DialogSelection {
    count: usize,
    current: usize,
}

impl DialogSelection {
    pub fn set_up(&mut self, count: usize) {
        self.count = count;
        self.current = 0;
    }

    pub fn up(&mut self) {
        if self.count != 0 {
            self.current = (self.current + self.count - 1) % self.count;
        } else {
            self.current = 0;
        }
    }

    pub fn down(&mut self) {
        if self.count != 0 {
            self.current = (self.current + 1) % self.count;
        } else {
            self.current = 0;
        }
    }

    pub fn current(&self) -> usize {
        self.current + 1
    }
}
