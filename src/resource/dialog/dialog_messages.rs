use std::sync::Mutex;
use inkgen::runtime::{Story, Paragraph};

#[derive(Default, Debug)]
pub struct DialogMessages {
    story: Mutex<Option<Story>>,
    paragraph: Option<Paragraph>
}

impl DialogMessages {
    pub fn start(&mut self, story: Story) {
        let (paragraph, story) = match story {
            Story::Unstarted(story) => story.start(),
            _ => panic!("The story to start must not be started"),
        };
        self.paragraph = Some(paragraph);
        *self.story.lock().unwrap() = Some(story);
    }

    pub fn current(&self) -> Option<&Paragraph> {
        self.paragraph.as_ref()
    }

    pub fn current_mut(&mut self) -> Option<&mut Paragraph> {
        self.paragraph.as_mut()
    }

    pub fn next(&mut self) -> Option<&Paragraph> {
        let mut story = self.story.lock().unwrap();
        match story.take() {
            None => self.paragraph = None,
            Some(Story::Ended(ended_story)) => {
                self.paragraph = None;
                *story = Some(Story::Ended(ended_story));
            }
            Some(Story::Unstarted(..)) => panic!("The story must be started already to continue"),
            Some(Story::Regular(regular_story)) => {
                let (paragraph, next_story) = regular_story.next();
                self.paragraph = Some(paragraph);
                *story = Some(next_story);
            }
        }
        self.current()
    }

    pub fn select(&mut self, option: usize) -> Option<&Paragraph> {
        let mut story = self.story.lock().unwrap();
        match story.take() {
            None => self.paragraph = None,
            Some(Story::Ended(ended_story)) => {
                self.paragraph = None;
                *story = Some(Story::Ended(ended_story));
            }
            Some(Story::Unstarted(..)) => panic!("The story must be started already to continue"),
            Some(Story::Regular(regular_story)) => {
                let (paragraph, next_story) = regular_story.select(option);
                self.paragraph = Some(paragraph);
                *story = Some(next_story);
            }
        }
        self.current()
    }

    pub fn is_empty(&self) -> bool {
        match &*self.story.lock().unwrap() {
            None | Some(Story::Ended(..)) => true,
            _ => false,
        }
    }
}
