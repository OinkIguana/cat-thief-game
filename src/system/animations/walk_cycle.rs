use component::graphics::{SpriteFrame, WalkCycle, AnimationSpeed};
use component::velocity::Velocity;
use component::position::{Position, PreviousPosition};

/// Animates a character's walk cycle. The speed is reduced by half when the character is not
/// moving at its expected velocity (such as when colliding with a wall). It is assumed that the
/// first frame of each direction is the idle frame, and then rest are used for the cycle.
///
/// TODO: might be nice to have the animation run faster when the character is running (moving at
/// double the BaseMovementSpeed) or even have a speed proportional to Velocity/BaseMovementSpeed,
/// which cuts off at a minimum of AnimationSpeed/2 so long as there is supposed to be some
/// Velocity (to keep walking into walls appearance)
#[derive(Default)]
pub struct AnimateWalkCycle;

system! {
    impl AnimateWalkCycle {
        fn run(
            &mut self,
            entities: &Entities,
            walk_cycle: &Component<WalkCycle>,
            position: &Component<Position>,
            previous_position: &Component<PreviousPosition>,
            velocity: &Component<Velocity>,
            animation_speed: &Component<AnimationSpeed>,
            sprite_frame: &mut Component<SpriteFrame>,
        ) {
            for (entity, walk_cycle, velocity, animation_speed, sprite_frame) in 
                (&*entities, &walk_cycle, &velocity, &animation_speed, &mut sprite_frame).join() {
                let image_speed = 
                    if let (Some(position), Some(previous_position)) = (position.get(entity), previous_position.get(entity)) {
                        let delta_pos = position.0 - previous_position.0;
                        if (delta_pos.x.powi(2) + delta_pos.y.powi(2)).sqrt() != velocity.magnitude() {
                            animation_speed.0 / 2f32
                        } else {
                            animation_speed.0
                        }
                    } else {
                        animation_speed.0
                    };

                if velocity.magnitude() == 0f32 || animation_speed.0 == 0f32 {
                    let frames = walk_cycle.directions
                        .iter()
                        .map(|(_, range)| range)
                        .find(|range| range.contains(&sprite_frame.current()));
                    if let Some(frames) = frames {
                        sprite_frame.0 = frames.start as f32;
                    }
                } else {
                    if let Some(frames) = walk_cycle.frames(velocity.direction().unwrap()) {
                        sprite_frame.0 += image_speed;
                        let walking_frames = frames.start + 1..frames.end;
                        if !walking_frames.contains(&sprite_frame.current()) {
                            sprite_frame.0 = walking_frames.start as f32;
                        }
                    }
                }
            }
        }
    }
}
