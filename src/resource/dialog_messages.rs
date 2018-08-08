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
        let (paragraph, story) = unsafe { story.next() };
        self.paragraph = Some(paragraph);
        *self.story.lock().unwrap() = story;
    }

    pub fn current(&self) -> Option<&Paragraph> {
        self.paragraph.as_ref()
    }

    pub unsafe fn next(&mut self) -> Option<&Paragraph> {
        let mut story = self.story.lock().unwrap();
        if story.is_none() {
            self.paragraph = None;
        } else {
            let (paragraph, next_story) = story.take()?.next();
            self.paragraph = Some(paragraph);
            *story = next_story;
        }
        self.current()
    }

    pub unsafe fn select(&mut self, option: usize) -> Option<&Paragraph> {
        let mut story = self.story.lock().unwrap();
        if story.is_none() {
            self.paragraph = None;
        } else {
            let (paragraph, next_story) = story.take()?.select(option);
            self.paragraph = Some(paragraph);
            *story = next_story;
        }
        self.current()
    }

    pub fn is_empty(&self) -> bool {
        self.story.lock().unwrap().is_none()
    }
}
