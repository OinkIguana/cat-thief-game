use game_engine::{system, prelude::*};

use crate::{
    component::{
        marker::{Pickup, Player},
        state_target::StateTarget,
        position::Position,
        collision_box::CollisionBox,
    },
    resource::state::State,
};

#[derive(Default, Debug)]
pub struct StatePickups;

system! {
    impl StatePickups {
        fn run(
            &mut self,
            entities: &Entities,
            delete: &mut Component<Delete>,
            player: &Component<Player>,
            pickup: &Component<Pickup>,
            state_target: &Component<StateTarget>,
            position: &Component<Position>,
            collision_box: &Component<CollisionBox>,
            state: &mut Resource<State>,
        ) {
            let player_box = {
                if let Some((_, position, collision_box)) = (&player, &position, &collision_box).join().next() {
                    collision_box.at(position.0)
                } else {
                    return
                }
            };
            for (entity, _, state_target, collision_box) in (&*entities, &pickup, &state_target, &collision_box).join() {
                if collision_box.0.overlaps(&player_box) {
                    state.main = state_target.0;
                    delete.insert(entity, Delete::default()).unwrap();
                }
            }
        }
    }
}
