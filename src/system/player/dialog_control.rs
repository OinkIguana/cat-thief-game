use game_engine::system;
use crate::resource::{
    constant::DialogSpeed,
    dialog_messages::{DialogProgress, DialogMessages},
    control_events::{ControlEvents, ControlEvent},
};

#[derive(Default, Debug)]
pub struct DialogControl;

system! {
    impl DialogControl {
        fn run(
            &mut self,
            control_events: &Resource<ControlEvents>,
            dialog_progress: &mut Resource<DialogProgress>,
            dialog_messages: &mut Resource<DialogMessages>,
            dialog_speed: &Resource<DialogSpeed>,
        ) {
            let paragraph = dialog_messages.current();
            if let Some(paragraph) = paragraph {
                if dialog_progress.current().is_some() {
                    dialog_progress.progress(dialog_speed.0, paragraph.text().len());
                }

                for event in control_events.iter() {
                    match event {
                        | &ControlEvent::Action(..)
                        | &ControlEvent::Cancel(..) => {
                            if dialog_progress.current().is_some() {
                                dialog_progress.skip();
                            } else {
                                dialog_progress.reset();
                                unsafe { dialog_messages.next() };
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
