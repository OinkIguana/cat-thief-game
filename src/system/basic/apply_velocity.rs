use engine::prelude::*;

use component::{
    position::{Position, PreviousPosition},
    velocity::Velocity,
    collision_box::CollisionBox,
};

#[derive(Default, Debug)]
pub struct ApplyVelocity;

system! {
    impl ApplyVelocity {
        fn run(
            &mut self,
            entities: &Entities,
            velocity: &Component<Velocity>,
            position: &mut Component<Position>,
            previous_position: &mut Component<PreviousPosition>,
            collision_box: &Component<CollisionBox>,
        ) {
            let collision_boxes: Vec<_> = (&*entities, &collision_box).join().collect();
            // NOTE: this might be a candidate for par_join, as it is potentially expensive, and
            // probably happening often
            for (entity, velocity, mut position) in (&*entities, &velocity, &mut position).join() {
                if let Some(mut previous_position) = previous_position.get_mut(entity) {
                    previous_position.0 = position.0;
                }
                if velocity.magnitude() == 0f32 {
                    continue;
                }
                let mut x_velocity = Point::new(velocity.0.x, 0f32);
                let mut y_velocity = Point::new(0f32, velocity.0.y);
                if let Some(collision_box) = collision_box.get(entity) {
                    for collision in collision_boxes.iter().filter(|(e, _)| *e != entity).map(|(_, b)| b) {
                        let mut final_x = collision_box.at(position.0 + x_velocity);
                        let mut final_y = collision_box.at(position.0 + y_velocity);
                        while collision.0.overlaps(&final_x) {
                            x_velocity.x -= x_velocity.x.signum() * f32::min(x_velocity.x.abs(), 1f32);
                            final_x = collision_box.at(position.0 + x_velocity);
                        }
                        while collision.0.overlaps(&final_y) {
                            y_velocity.y -= y_velocity.y.signum() * f32::min(y_velocity.y.abs(), 1f32);
                            final_y = collision_box.at(position.0 + y_velocity);
                        }
                    }
                    position.0 = position.0 + x_velocity + y_velocity;
                } else {
                    position.0 = position.0 + velocity.0;
                }
            }
        }
    }
}
