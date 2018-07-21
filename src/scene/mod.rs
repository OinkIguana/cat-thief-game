use engine::prelude::*;

use constant::TILE_SIZE;
use system::{
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
use resource::dialog_messages::DialogMessages;
use model::{
    message::Message,
    pretty_string::{PrettyString, Attribute}
};
use entity::{
    meta::Dialog,
    player::Player,
};
use tile_grid::town;

scene! {
    pub START {
        entities: [
            Dialog,
            Player(TILE_SIZE * 20 + TILE_SIZE / 2, 10 * TILE_SIZE),
        ],
        systems: [
            (PlayerMovement::default(), "PlayerMovement", &[]),
            (ApplyVelocity::default(), "ApplyVelocity", &["PlayerMovement"]),
            (CameraTarget::default(), "CameraTarget", &["ApplyVelocity"]),
            (AnimateWalkCycle::default(), "AnimateWalkCycle", &["ApplyVelocity"]),
            (MaintainSpriteDrawable::default(), "MaintainSpriteDrawable", &["AnimateWalkCycle"]),
            (DialogControl::default(), "DialogControl", &[]),
            (MaintainDialogDrawable::default(), "MaintainDialogDrawable", &["DialogControl"]),
        ]
    } => |builder| {
        {
            let mut layers = builder.get_resource_mut::<TileLayers>();
            layers.set(-5, town::WATER.clone());
            layers.set(-4, town::GROUND.clone());
            layers.set(-3, town::WALLS.clone());
            layers.set(-2, town::OBSTACLES.clone());
            layers.set(-1, town::DOORS.clone());
            layers.set(1, town::ROOFS.clone());
        }
        builder.get_resource_mut::<DialogMessages>().add("Hello this is a message");
        builder.get_resource_mut::<DialogMessages>().add("Hello this is a message\nHahaha it's two lines long");
        builder.get_resource_mut::<DialogMessages>().add("lalala this is super long lalalal ahahahah librlaib laihe bliahs eliah slibha lsiehg alishe glaishe liashe glgiah segligah selifha lsiehf falieh alishe fliahse efiahs eflfiahe leifha sleihf a");
        builder.get_resource_mut::<DialogMessages>().add(
            Message::new(
                "Cameron".to_owned(), 
                PrettyString::default()
                    .add("I am looking for this ")
                    .add(("thing", vec![Attribute::Color(Color::BLUE)]))
            )
        );
        builder.pipe(town::collisions)
    }
}
