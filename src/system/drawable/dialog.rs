use game_engine::{system, prelude::*};
use crate::drawable::DialogDrawable;
use crate::resource::dialog_messages::{DialogMessages, DialogProgress, DialogSelection};

#[derive(Default, Debug)]
pub struct MaintainDialogDrawable;

system! {
    impl MaintainDialogDrawable {
        fn run(
            &mut self,
            drawable: &mut Component<Box<dyn Drawable>>,
            dialog_messages: &Resource<DialogMessages>,
            dialog_progress: &Resource<DialogProgress>,
            dialog_selection: &Resource<DialogSelection>,
        ) {
            for drawable in (&mut drawable).join() {
                if let Some(drawable) = drawable.as_any_mut().downcast_mut::<DialogDrawable>() {
                    drawable.index = dialog_progress.current();
                    drawable.paragraph = dialog_messages.current().cloned();
                    drawable.selection = dialog_selection.current() - 1;
                }
            }
        }
    }
}
