use game_engine::prelude::*;
use crate::resource::cutscene::CurrentCutscene;

pub(super) fn process_cutscene(world: &mut World) {
    let cutscene = world.write_resource::<CurrentCutscene>().take();
    if let Some(mut cutscene) = cutscene {
        cutscene.run(world);
        world.write_resource::<CurrentCutscene>().start(cutscene);
    }
}
