use model::direction::Direction;
use component::{
    behavior::MovePath,
    position::Position,
    velocity::Velocity,
};
use resource::constant::BaseMovementSpeed;

#[derive(Default, Debug)]
pub struct MoveByMovePath;

system! {
    impl MoveByMovePath {
        fn run(
            &mut self,
            position: &Component<Position>,
            velocity: &mut Component<Velocity>,
            move_path: &mut Component<MovePath>,
            base_movement_speed: &Resource<BaseMovementSpeed>,
        ) {
            'entities: for (position, mut move_path, mut velocity) in (&position, &mut move_path, &mut velocity).join() {
                if let Some(mut target) = move_path.target() {
                    while target == position.0 {
                        move_path.arrive();
                        match move_path.target() {
                            Some(t) => target = t,
                            None => continue 'entities,
                        }
                    }
                    let direction = Direction::from_origin(target - position.0).unwrap();
                    let distance = (position.0 - target).magnitude();
                    *velocity = Velocity::angular(f32::min(base_movement_speed.0 as f32, distance), direction);
                }
            }
        }
    }
}

