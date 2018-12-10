#![feature(const_fn, range_contains, generators, generator_trait, in_band_lifetimes)]
#![deny(bare_trait_objects)]

pub mod component;
pub mod constant;
pub mod cutscene;
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
                .with(PlayerMovement::default(), "PlayerMovement", &[])
                .with(MoveByMovePath::default(), "MoveByMovePath", &["PlayerMovement"])
                .with(ApplyVelocity::default(), "ApplyVelocity", &["MoveByMovePath"])
                .with(CameraTarget::default(), "CameraTarget", &["ApplyVelocity"])
                .with(AnimateWalkCycle::default(), "AnimateWalkCycle", &["ApplyVelocity"])
                .with(EnterDoors::default(), "EnterDoors", &["ApplyVelocity"])
                .with(StatePickups::default(), "StatePickups", &["ApplyVelocity"])
                .with(MaintainSpriteDrawable::default(), "MaintainSpriteDrawable", &["AnimateWalkCycle"])
                .with(MaintainDialogDrawable::default(), "MaintainDialogDrawable", &[])
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
