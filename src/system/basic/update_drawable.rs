use std::any::Any;
use engine::prelude::*;
use drawable;
use component::{
    position::Position,
    graphics::{SpriteFrame, DrawDepth},
};

#[derive(Default, Debug)]
pub struct UpdateDrawable;

system! {
    impl UpdateDrawable {
        fn run(
            &mut self,
            entities: &Entities,
            position: &Component<Position>,
            draw_depth: &Component<DrawDepth>,
            sprite_frame: &Component<SpriteFrame>,
            drawable: &mut Component<Box<dyn Drawable>>,
        ) {
            for (entity, mut drawable) in (&*entities, &mut drawable).join() {
                if let Some(drawable) = Any::downcast_mut::<drawable::SpriteDrawable>(drawable.as_any_mut()) {
                    if let Some(position) = position.get(entity) {
                        drawable.position = position.rounded();
                    }
                    if let Some(frame) = sprite_frame.get(entity) {
                        drawable.frame = frame.current();
                    }
                    if let Some(depth) = draw_depth.get(entity) {
                        drawable.depth = depth.0;
                    }
                }
            }
        }
    }
}
