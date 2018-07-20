use std::any::Any;
use engine::{self, prelude::*};
use model::message::Message;

#[derive(Default, Debug)]
pub struct DialogDrawable {
    pub index: Option<usize>,
    pub message: Option<Message>,
}

impl DialogDrawable {
    pub fn boxed() -> Box<dyn Drawable> {
        Box::new(Self::default()) 
    }
}

impl Drawable for DialogDrawable {
    fn depth(&self) -> i32 {
        ::std::i32::MAX
    }

    fn render(&self, canvas: &mut dyn Canvas) -> engine::Result<()> {
        if let Some(message) = &self.message {
            // TODO: draw a nice dialog box thing
            let segments = 
                if let Some(index) = self.index {
                    message.message().up_to(index)
                } else {
                    message.message().clone()
                };
            for (string, attributes) in &segments.0 {
                // TODO: draw the string segments at the right positions
            }
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
