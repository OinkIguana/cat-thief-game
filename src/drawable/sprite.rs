use engine::{self, prelude::*};

pub struct SpriteDrawable {
    pub depth: i32,
    pub position: Point,
    pub frame: usize,
    pub sprites: Vec<&'static Sprite>,
}

impl SpriteDrawable {
    pub fn boxed(sprites: Vec<&'static Sprite>) -> Box<dyn Drawable> {
        Box::new(SpriteDrawable { 
            depth: 0,
            position: Point::default(), 
            frame: 0, 
            sprites,
        })
    }
}

use std::any::Any;

impl Drawable for SpriteDrawable {
    fn depth(&self) -> i32 {
        self.depth
    }

    fn render(&self, canvas: &mut dyn Canvas) -> engine::Result<()> {
        for sprite in &self.sprites {
            canvas.draw_sprite(self.position, self.frame, (*sprite).clone())?;
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
