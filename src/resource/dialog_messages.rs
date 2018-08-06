use std::sync::Mutex;
use inkgen::runtime::{Story, Paragraph};

#[derive(Copy, Clone, PartialEq, Debug)]
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

#[derive(Default, Debug)]
pub struct DialogMessages {
    story: Mutex<Option<Story>>,
    paragraph: Option<Paragraph>
}

impl DialogMessages {
    pub fn start(&mut self, story: Story) {
        if let Some((paragraph, story)) = unsafe { story.next() } {
            self.paragraph = Some(paragraph);
            *self.story.lock().unwrap() = Some(story);
        }
    }

    pub fn current(&self) -> Option<&Paragraph> {
        self.paragraph.as_ref()
    }

    pub unsafe fn next(&mut self) -> Option<&Paragraph> {
        let mut story = self.story.lock().unwrap();
        if let Some((paragraph, next_story)) = story.take()?.next() {
            self.paragraph = Some(paragraph);
            *story = Some(next_story);
        } else {
            self.paragraph = None;
            *story = None;
        }
        self.current()
    }

    pub unsafe fn select(&mut self, option: usize) -> Option<&Paragraph> {
        let mut story = self.story.lock().unwrap();
        if let Some((paragraph, next_story)) = story.take()?.select(option) {
            self.paragraph = Some(paragraph);
            *story = Some(next_story);
        } else {
            self.paragraph = None;
            *story = None;
        }
        self.current()
    }

    pub fn is_empty(&self) -> bool {
        self.story.lock().unwrap().is_none()
    }
}
