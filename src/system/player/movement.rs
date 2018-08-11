use game_engine::{system, prelude::*};
use crate::component::{
    marker::Player,
    velocity::Velocity,
};
use crate::resource::{
    constant::BaseMovementSpeed,
    control::ControlState,
    dialog::DialogMessages,
    cutscene::CurrentCutscene,
};

#[derive(Default, Debug)]
pub struct PlayerMovement;

system! {
    impl PlayerMovement {
        fn run(
            &mut self,
            velocity: &mut Component<Velocity>,
            player: &Component<Player>,
            control_state: &Resource<ControlState>,
            base_movement_speed: &Resource<BaseMovementSpeed>,
            current_cutscene: &Resource<CurrentCutscene>,
            dialog_messages: &Resource<DialogMessages>,
        ) {
            let axis_h = control_state.axis_h as f32;
            let axis_v = control_state.axis_v as f32;
            let running = control_state.run;
            let scale = ::std::i8::MAX as f32;
            let movement_speed = if running { base_movement_speed.0 * 2 } else { base_movement_speed.0 } as f32;
            let hspeed = axis_h / scale * movement_speed;
            let vspeed = axis_v / scale * movement_speed;

            for (_, velocity) in (&player, &mut velocity).join() {
                if dialog_messages.current().is_some() || !current_cutscene.is_over() {
                    // disable player control while dialog is visible
                    velocity.0 = Point::default();
                } else {
                    velocity.0 = Point::new(hspeed, vspeed);
                }
            }
        }
    }
}
