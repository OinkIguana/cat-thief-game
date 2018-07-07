use component::position::Position;
use component::velocity::Velocity;

#[derive(Default, Debug)]
pub struct ApplyVelocity;

system! {
    impl ApplyVelocity {
        fn run(
            &mut self,
            velocity: &Component<Velocity>,
            position: &mut Component<Position>,
        ) {
            for (velocity, mut position) in (&velocity, &mut position).join() {
                position.0 = position.0 + velocity.0;
            }
        }
    }
}
