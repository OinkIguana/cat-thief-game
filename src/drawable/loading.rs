use std::any::Any;
use engine::{self, prelude::*};

use font::abyssinica::REGULAR_18 as abyssinica;

#[derive(Default, Debug)]
pub struct LoadingDrawable {
    pub visible: bool,
    pub dots: f32,
}

impl LoadingDrawable {
    pub fn boxed() -> Box<dyn Drawable> {
        Box::new(Self::default())
    }
}

impl Drawable for LoadingDrawable {
    fn depth(&self) -> i32 {
        ::std::i32::MAX
    }

    fn render(&self, canvas: &mut dyn Canvas) -> engine::Result<()> {
        if !self.visible { return Ok(()); }

        let size = canvas.size();
        canvas.set_font(abyssinica);
        canvas.set_transform(Rect::from(Point::default(), size), Rect::from(Point::default(), size));
        canvas.set_color(Color::BLACK);
        canvas.draw_rect_filled(Rect::from(Point::default(), size))?;
        canvas.set_color(Color::WHITE);

        let Dimen { width, height } = canvas.measure_text("Loading...".to_string())?;

        canvas.draw_text(Point::new(size.width as i32 - width as i32 - 32, size.height as i32 - height as i32 - 32), format!("Loading{}", ".".repeat(self.dots as usize)))?;

        Ok(())
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
