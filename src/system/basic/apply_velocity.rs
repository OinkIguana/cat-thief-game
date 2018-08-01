use game_engine::{system, prelude::*};

use crate::component::{
    marker::Solid,
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
            solid: &Component<Solid>,
        ) {
            let collision_boxes: Vec<_> = (&*entities, &collision_box, &solid)
                .join()
                .map(|(entity, collision_box, _)| {
                    if let Some(position) = position.get(entity) {
                        (entity, collision_box.at(position.0))
                    } else {
                        (entity, collision_box.0)
                    }
                })
                .collect();
            // NOTE: this might be a candidate for par_join, as it is potentially expensive, and
            // probably happening often
            for (entity, velocity, position) in (&*entities, &velocity, &mut position).join() {
                if let Some(previous_position) = previous_position.get_mut(entity) {
                    previous_position.0 = position.0;
                }
                if velocity.magnitude() == 0f32 {
                    continue;
                }
                let mut final_velocity = velocity.0;
                if let (Some(_), Some(collision_box)) = (solid.get(entity), collision_box.get(entity)) {
                    let collisions: Vec<_> = {
                        let destination = collision_box.at(position.0 + final_velocity);
                        collision_boxes
                            .iter()
                            .filter(|(e, _)| *e != entity)
                            .map(|(_, b)| b)
                            .filter(|b| b.overlaps(&destination))
                            .collect()
                    };

                    let mut x_velocity = 0f32;
                    let mut y_velocity = 0f32;
                    'find_x: for dx in 0..=final_velocity.x.abs() as u32 {
                        let x_vel = Point::new(final_velocity.x.signum() * dx as f32, 0f32);
                        let destination = collision_box.at(position.0+ x_vel);
                        for collision in &collisions {
                            if destination.overlaps(&collision) {
                                break 'find_x;
                            }
                        }
                        x_velocity = x_vel.x;
                    }
                    'find_y: for dy in 0..=final_velocity.y.abs() as u32 {
                        let y_vel = Point::new(x_velocity, final_velocity.y.signum() * dy as f32);
                        let destination = collision_box.at(position.0 + y_vel);
                        for collision in &collisions {
                            if destination.overlaps(&collision) {
                                break 'find_y;
                            }
                        }
                        y_velocity = y_vel.y;
                    }
                    final_velocity = Point::new(x_velocity, y_velocity);
                }
                position.0 = position.0 + final_velocity;
            }
        }
    }
}
