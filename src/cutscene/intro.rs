use game_engine::prelude::*;
use serde::Deserialize;
use crate::constant::TILE_SIZE;
use crate::model::cutscene::{Cutscene, StandardCutscene, Step::*};
use crate::component::id::Id;
use crate::dialog::intro as dialog;

#[derive(Deserialize, PartialEq)]
enum EnterAlleyEvent {
    ComeOut,
}

pub fn enter_alley() -> Box<dyn Cutscene> {
    StandardCutscene::boxed(&[
        Move(Id::Player, &[Point::new(TILE_SIZE as f32 * 24f32, TILE_SIZE as f32 * 11f32)]),
        Break,
        AwaitMoveEnd(Id::Player),
        StartDialog(dialog::enter_alley::story),
        AwaitDialogEvent(EnterAlleyEvent::ComeOut),
        Move(Id::MysteryMan, &[Point::new(TILE_SIZE as f32 * 19f32, TILE_SIZE as f32 * 11f32)]),
    ])
}
