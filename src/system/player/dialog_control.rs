use game_engine::system;
use inkgen::runtime::Paragraph;
use crate::resource::{
    constant::DialogSpeed,
    dialog_messages::{DialogProgress, DialogMessages, DialogSelection},
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
            dialog_selection: &mut Resource<DialogSelection>,
            dialog_speed: &Resource<DialogSpeed>,
        ) {
            let paragraph = dialog_messages.current().cloned();
            if let Some(paragraph) = paragraph {
                if dialog_progress.current().is_some() {
                    dialog_progress.progress(dialog_speed.0, paragraph.text().len());
                }

                for event in control_events.iter() {
                    match event {
                        ControlEvent::Down(..) => dialog_selection.down(),
                        ControlEvent::Up(..) => dialog_selection.down(),

                        | ControlEvent::Action(..)
                        | ControlEvent::Cancel(..) => {
                            if dialog_progress.current().is_some() {
                                dialog_progress.skip();
                            } else {
                                dialog_progress.reset();
                                let current =
                                    if paragraph.choices().is_some() {
                                        dialog_messages.select(dialog_selection.current())
                                    } else {
                                        dialog_messages.next()
                                    };
                                if let Some(count) = current.and_then(Paragraph::choices).map(|choices| choices.len()){
                                    dialog_selection.set_up(count);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
