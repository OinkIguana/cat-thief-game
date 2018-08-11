use game_engine::system;
use crate::resource::{
    cutscene::CutsceneActions,
    dialog::DialogMessages,
};
use crate::component::{
    id::Id,
    behavior::MovePath,
};
use crate::model::cutscene::Action;

#[derive(Default, Debug)]
pub struct RunCutscene;

system! {
    impl RunCutscene {
        fn run(
            &mut self,
            entities: &Entities,
            cutscene_actions: &Resource<CutsceneActions>,
            dialog_messages: &mut Resource<DialogMessages>,
            id: &Component<Id>,
            move_path: &mut Component<MovePath>,
        ) {
            for action in cutscene_actions.iter() {
                match action {
                    Action::Dialog(dialog) => dialog_messages.start(dialog()),
                    Action::Move(target, path) => {
                        for (entity, id) in (&*entities, &id).join() {
                            if id == target {
                                move_path.insert(entity, MovePath::from(*path)).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
