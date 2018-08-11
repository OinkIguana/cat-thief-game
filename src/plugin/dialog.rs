use game_engine::prelude::*;
use crate::resource::{
    dialog::{DialogSpeed, DialogProgress, DialogMessages, DialogSelection, DialogEvents},
    control::{ControlEvents, ControlEvent},
};

pub(super) fn manage_dialog(world: &mut World) {
    let control_events = world.read_resource::<ControlEvents>();
    let mut dialog_progress = world.write_resource::<DialogProgress>();
    let mut dialog_messages = world.write_resource::<DialogMessages>();
    let mut dialog_events = world.write_resource::<DialogEvents>();
    let mut dialog_selection = world.write_resource::<DialogSelection>();
    let dialog_speed = world.read_resource::<DialogSpeed>();

    dialog_events.clear();
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
                        if paragraph.choices().is_some() {
                            dialog_messages.select(dialog_selection.current())
                        } else {
                            dialog_messages.next()
                        };
                        if let Some(current) = dialog_messages.current_mut() {
                            if let Some(count) = current.choices().as_ref().map(Vec::len) {
                                dialog_selection.set_up(count);
                            }
                            for tag in current.take_tags().into_iter() {
                                dialog_events.add(tag);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
