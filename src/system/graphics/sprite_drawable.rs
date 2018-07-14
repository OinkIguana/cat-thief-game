use engine::prelude::*;
use component::graphics::{Sprite, SpriteFrame, SpriteLayers};

#[derive(Default, Debug)]
pub struct SpriteDrawable;

system! {
    impl SpriteDrawable {
        fn run(
            &mut self,
            entities: &Entities,
            sprite: &Component<Sprite>,
            sprite_layers: &Component<SpriteLayers>,
            sprite_frame: &Component<SpriteFrame>,
            drawable: &mut Component<Drawable>,
        ) {
            for (entity, sprite_layers, mut drawable) in (&*entities, &sprite_layers, &mut drawable).join() {
                let frame = sprite_frame.get(entity).map(|frame| frame.current()).unwrap_or(0);
                let position = drawable.position();
                let mut builder = Drawable::new();
                for layer in sprite_layers.iter() {
                    builder = builder
                        .sprite(layer.clone())
                        .frame(frame)
                        .commit();
                }
                *drawable = builder.build();
                drawable.set_position(position);
            }
            for (entity, sprite, mut drawable) in (&*entities, &sprite, &mut drawable).join() {
                let frame = sprite_frame.get(entity).map(|frame| frame.current()).unwrap_or(0);
                let position = drawable.position();
                *drawable = Drawable::new()
                    .sprite(sprite.0.clone())
                    .frame(frame)
                    .build();
                drawable.set_position(position);
            }
        }
    }
}
