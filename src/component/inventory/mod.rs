use game_engine::prelude::Game;

mod wallet;
mod inventory;

pub use self::{
    wallet::Wallet,
    inventory::Inventory,
};

pub(super) fn register<'a, 'b>(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.register_component::<Wallet>()
        .register_component::<Inventory>()
}
