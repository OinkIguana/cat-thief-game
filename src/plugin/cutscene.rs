use game_engine::prelude::*;
use crate::resource::cutscene::{CutsceneActions, CurrentCutscene};

pub(super) fn process_cutscene(world: &mut World) {
    world.write_resource::<CutsceneActions>().clear();
    let mut current_cutscene = world.write_resource::<CurrentCutscene>();
    if let Some(actions) = current_cutscene.actions(world) {
        world.write_resource::<CutsceneActions>().set(actions);
    }
}
