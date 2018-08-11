use game_engine::system;
use crate::resource::{
    cutscene::CurrentCutscene,
    state::{State, MainState},
};
use crate::cutscene::intro::enter_alley;

#[derive(Default, Debug)]
pub struct IntroSceneTriggers;

system! {
    impl IntroSceneTriggers {
        fn run(
            &mut self,
            state: &Resource<State>,
            current_cutscene: &mut Resource<CurrentCutscene>,
        ) {
            if state.just_started(MainState::ArriveInTheAlley) {
                current_cutscene.start(enter_alley());
            }
        }
    }
}
