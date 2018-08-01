#![feature(range_contains, const_fn, rust_2018_preview)]
#![deny(bare_trait_objects)]
#![warn(rust_2018_idioms)]
#![allow(dead_code, unreachable_pub)] // while still in early development, there's a lot of stuff unused.

mod component;
mod constant;
mod dialog;
mod drawable;
mod entity;
mod font;
mod image;
mod model;
mod plugin;
mod resource;
mod scene;
mod sprite;
mod system;
mod tile_grid;
mod tile_set;

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
                .with(MaintainSpriteDrawable::default(), "MaintainSpriteDrawable", &["AnimateWalkCycle"])
                .with(MaintainDialogDrawable::default(), "MaintainDialogDrawable", &["DialogControl"])
                .build()
        )

        .add_conditional_dispatcher(|world| world.read_resource::<IsLoading>().0, |builder|
            builder
                .with(ShowLoader::default(), "ShowLoader", &[])
                .build()
        )

        .add_dispatcher(|builder|
            builder
                .with(MaintainLoadingDrawable::default(), "MaintainLoadingDrawable", &[])
                .build()
        )

        .start(scene::town::outside::TOWN_OUTSIDE)
}
