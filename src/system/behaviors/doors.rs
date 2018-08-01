use game_engine::{system, prelude::*};
use crate::component::{
    marker,
    position::Position,
    collision_box::CollisionBox,
    behavior::MovePath,
    door::*,
};
use crate::resource::door_transition::DoorTransition;
use crate::entity::player::Player;

#[derive(Default, Debug)]
pub struct EnterDoors;

system! {
    impl<'a> EnterDoors {
        fn run(
            &mut self,
            entities: &Entities,
            scene_manager: &mut SceneManager<'a>,
            position: &Component<Position>,
            collision_box: &Component<CollisionBox>,
            player: &Component<marker::Player>,
            door_id: &Component<DoorID>,
            target_scene: &Component<TargetScene>,
            move_path: &Component<MovePath>,
            door_transition: &mut Resource<DoorTransition>,
        ) {
            for (_, entity, player_position, player_box) in (&player, &*entities, &position, &collision_box).join() {
                if move_path.get(entity).map(|p| !p.is_empty()).unwrap_or(false) {
                    // ignore doors if the player is moving by move path
                    return;
                }
                let player_box = player_box.at(player_position.0);
                for (door_id, target_scene, position, collision_box) in (&door_id, &target_scene, &position, &collision_box).join() {
                    let door_box = collision_box.at(position.0);
                    if player_box.overlaps(&door_box) {
                        scene_manager.change(target_scene.0);
                        door_transition.to(*door_id);
                    }
                }
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct ExitDoors;

system! {
    impl ExitDoors {
        fn run(
            &mut self,
            entities: &Entities,
            lazy_update: &LazyUpdate,
            door_id: &Component<DoorID>,
            door_exit: &Component<DoorExit>,
            position: &Component<Position>,
            door_transition: &mut Resource<DoorTransition>,
        ) {
            if let Some(target_id) = door_transition.take_target() {
                for (door_id, position, door_exit) in (&door_id, &position, &door_exit).join() {
                    if &target_id == door_id {
                        Player(position.rounded().x, position.rounded().y)
                            .build(lazy_update.create_entity(&entities))
                            .with(SceneMember)
                            .with(door_exit.move_path(position.0))
                            .build();
                        break;
                    }
                }
            }
        }
    }
}
