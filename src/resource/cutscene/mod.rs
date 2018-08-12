use game_engine::prelude::*;
mod current_cutscene;
pub use self::current_cutscene::CurrentCutscene;

pub(super) fn register(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.add_resource(CurrentCutscene::default())
}
