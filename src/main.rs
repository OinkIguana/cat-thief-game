#![feature(range_contains, const_fn, generators, generator_trait, pattern_parentheses)]
#![deny(bare_trait_objects)]
#![warn(rust_2018_idioms)]

pub mod component;
pub mod constant;
pub mod dialog;
pub mod drawable;
pub mod entity;
pub mod font;
pub mod image;
pub mod model;
pub mod plugin;
pub mod resource;
pub mod scene;
pub mod sprite;
pub mod system;
pub mod tile_grid;
pub mod tile_set;

use game_engine::prelude::*;

use crate::system::{
    behaviors::{
        move_path::MoveByMovePath,
        doors::EnterDoors,
    },
    player::{
        dialog_control::DialogControl,
        movement::PlayerMovement,
    },
    basic::{
        apply_velocity::ApplyVelocity,
        camera_target::CameraTarget,
        loader::{HideLoader, ShowLoader},
        state_pickups::StatePickups,
    },
    drawable::{
        sprite::MaintainSpriteDrawable,
        dialog::MaintainDialogDrawable,
        loading::MaintainLoadingDrawable,
    },
    animations::AnimateWalkCycle,
};

fn main() -> game_engine::Result<()> {
    Game::new()
        .titled("Fun cat game")
        .target_fps(60)

        .pipe(component::register)
        .pipe(resource::register)
        .pipe(plugin::register)

        .add_conditional_dispatcher(|world| !world.read_resource::<IsLoading>().0, |builder|
            builder
                .with(HideLoader::default(), "HideLoader", &[])
                .with(DialogControl::default(), "DialogControl", &[])
                .with(MoveByMovePath::default(), "MoveByMovePath", &[])
                .with(PlayerMovement::default(), "PlayerMovement", &["DialogControl", "MoveByMovePath"])
                .with(ApplyVelocity::default(), "ApplyVelocity", &["PlayerMovement"])
                .with(CameraTarget::default(), "CameraTarget", &["ApplyVelocity"])
                .with(AnimateWalkCycle::default(), "AnimateWalkCycle", &["ApplyVelocity"])
                .with(EnterDoors::default(), "EnterDoors", &["ApplyVelocity"])
                .with(StatePickups::default(), "StatePickups", &["ApplyVelocity"])
                .with(MaintainSpriteDrawable::default(), "MaintainSpriteDrawable", &["AnimateWalkCycle"])
                .with(MaintainDialogDrawable::default(), "MaintainDialogDrawable", &["DialogControl"])
                .build()
        )

        .add_conditional_dispatcher(|world| world.read_resource::<IsLoading>().0, |builder|
            builder
                .with(ShowLoader::default(), "ShowLoader", &[])
                .build()
        )

        .add_dispatcher(system::state::dispatcher)

        .add_dispatcher(|builder|
            builder
                .with(MaintainLoadingDrawable::default(), "MaintainLoadingDrawable", &[])
                .build()
        )

        .start(scene::town::outside::TOWN_OUTSIDE)
}
