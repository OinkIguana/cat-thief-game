use game_engine::prelude::*;

pub(super) fn watch_resource<T: std::fmt::Debug + Sync + Send + 'static>(world: &mut World) {
    println!("{:?}", *world.read_resource::<T>())
}
