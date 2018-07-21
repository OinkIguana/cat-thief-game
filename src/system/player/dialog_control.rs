use resource::{
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
            let message_len = dialog_messages.current().map(|msg| msg.len());
            if let Some(message_len) = message_len {
                if dialog_progress.current().is_some() {
                    dialog_progress.progress(dialog_speed.0, message_len);
                }
            }
            if dialog_messages.current().is_some() {
                for event in control_events.iter() {
                    match event {
                        | &ControlEvent::Action(..) 
                        | &ControlEvent::Cancel(..) => {
                            if dialog_progress.current().is_some() {
                                dialog_progress.skip();
                            } else {
                                dialog_progress.reset();
                                dialog_messages.dismiss();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
