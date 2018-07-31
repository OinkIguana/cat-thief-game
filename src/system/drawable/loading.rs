use engine::prelude::*;
use drawable::LoadingDrawable;
use component::graphics::Visible;

#[derive(Default, Debug)]
pub struct MaintainLoadingDrawable;

system! {
    impl MaintainLoadingDrawable {
        fn run(
            &mut self,
            entities: &Entities,
            drawable: &mut Component<Box<dyn Drawable>>,
            visible: &Component<Visible>,
        ) {
            for (entity, mut drawable) in (&*entities, &mut drawable).join() {
                if let Some(ref mut drawable) = drawable.as_any_mut().downcast_mut::<LoadingDrawable>() {
                    drawable.dots += 0.05;
                    if drawable.dots >= 4f32 {
                        drawable.dots = 0f32;
                    }
                    drawable.visible = visible.get(entity).map(|visible| visible.0).unwrap_or(true);
                }
            }
        }
    }
}
