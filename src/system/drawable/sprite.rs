use game_engine::{system, prelude::*};
use crate::drawable::SpriteDrawable;
use crate::component::{
    position::Position,
    graphics::{SpriteFrame, SpriteOrigin, DrawDepth},
};

#[derive(Default, Debug)]
pub struct MaintainSpriteDrawable;

system! {
    impl MaintainSpriteDrawable {
        fn run(
            &mut self,
            entities: &Entities,
            position: &Component<Position>,
            draw_depth: &Component<DrawDepth>,
            sprite_frame: &Component<SpriteFrame>,
            sprite_origin: &Component<SpriteOrigin>,
            drawable: &mut Component<Box<dyn Drawable>>,
        ) {
            for (entity, drawable) in (&*entities, &mut drawable).join() {
                if let Some(drawable) = drawable.as_any_mut().downcast_mut::<SpriteDrawable>() {
                    if let Some(position) = position.get(entity) {
                        drawable.position = position.rounded();
                    } else {
                        drawable.position = Point::default();
                    }
                    if let Some(frame) = sprite_frame.get(entity) {
                        drawable.frame = frame.current();
                    }
                    if let Some(depth) = draw_depth.get(entity) {
                        drawable.depth = depth.0;
                    }
                    if let Some(origin) = sprite_origin.get(entity) {
                        drawable.position = drawable.position - origin.0;
                    }
                }
            }
        }
    }
}
