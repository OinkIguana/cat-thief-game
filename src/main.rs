#![feature(macro_at_most_once_rep, range_contains, const_fn)]
#![deny(bare_trait_objects)]
#![allow(dead_code)] // while still in early development, there's a lot of stuff unused.

#[macro_use] extern crate game_engine as engine;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate specs;
#[macro_use] extern crate specs_derive;
#[macro_use] extern crate lazy_static;

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

use engine::prelude::*;

use system::{
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
    },
    drawable::{
        sprite::MaintainSpriteDrawable,
        dialog::MaintainDialogDrawable,
    },
    animations::AnimateWalkCycle,
};

fn main() -> engine::Result<()> {
    Game::new()
        .pipe(component::register)
        .pipe(resource::register)
        .pipe(plugin::register)

        .add_conditional_dispatcher(|world| world.read_resource::<IsLoading>().0, |builder|
            builder.build()
        )

        .add_conditional_dispatcher(|world| !world.read_resource::<IsLoading>().0, |builder|
            builder
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

        .start(scene::town::outside::TOWN_OUTSIDE)
}
