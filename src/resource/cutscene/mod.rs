use game_engine::prelude::*;
mod current_cutscene;
mod cutscene_actions;
pub use self::{
    current_cutscene::CurrentCutscene,
    cutscene_actions::CutsceneActions,
};

pub(super) fn register(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.add_resource(CurrentCutscene::default())
        .add_resource(CutsceneActions::default())
}
