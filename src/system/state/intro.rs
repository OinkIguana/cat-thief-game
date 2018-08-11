use game_engine::system;
use crate::resource::{
    dialog::DialogMessages,
    state::{State, MainState},
};
use crate::dialog::intro::enter_alley;

#[derive(Default, Debug)]
pub struct IntroSceneTriggers;

system! {
    impl IntroSceneTriggers {
        fn run(
            &mut self,
            state: &Resource<State>,
            dialog_messages: &mut Resource<DialogMessages>,
        ) {
            if state.just_started(MainState::ArriveInTheAlley) {
                dialog_messages.start(enter_alley::story());
            }
        }
    }
}
