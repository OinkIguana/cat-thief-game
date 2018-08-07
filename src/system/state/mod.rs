use game_engine::prelude::*;
use game_engine::system;
use crate::resource::state::State;

mod intro;
use self::intro::IntroSceneTriggers;

#[derive(Default, Debug)]
struct ProcessState;

system! {
    impl ProcessState {
        fn run(
            &mut self,
            state: &mut Resource<State>,
        ) {
            state.process();
        }
    }
}

pub fn dispatcher(builder: DispatcherBuilder<'a, 'b>) -> Dispatcher<'a, 'b> {
    builder
        .with(IntroSceneTriggers::default(), "IntroSceneTriggers", &[])
        .with_barrier()
        .with(ProcessState::default(), "ProcessState", &[])
        .build()
}
