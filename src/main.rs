#![feature(macro_at_most_once_rep)]

#[macro_use] extern crate game_engine as engine;

mod scene;

use engine::Game;

fn main() -> engine::Result<()> {
    Game::new()
        .start(scene::START)
}
