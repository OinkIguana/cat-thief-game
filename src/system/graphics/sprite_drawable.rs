use engine::prelude::*;
use component::graphics::{Sprite, SpriteFrame};

#[derive(Default, Debug)]
pub struct SpriteDrawable;

system! {
    impl SpriteDrawable {
        fn run(
            &mut self,
            entities: &Entities,
            sprite: &Component<Sprite>,
            sprite_frame: &Component<SpriteFrame>,
            drawable: &mut Component<Drawable>,
        ) {
            for (entity, sprite, mut drawable) in (&*entities, &sprite, &mut drawable).join() {
                let position = drawable.position();
                *drawable = Drawable::new()
                    .sprite(sprite.0.clone())
                    .frame(sprite_frame.get(entity).map(|frame| frame.current()).unwrap_or(0))
                    .build();
                drawable.set_position(position);
            }
        }
    }
}
