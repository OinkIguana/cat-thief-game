use std::any::Any;
use engine::{self, prelude::*};

use model::{
    message::Message,
    pretty_string::Attribute,
};
use font::abyssinica::REGULAR_18 as DEFAULT_FONT;

#[derive(Default, Debug)]
pub struct DialogDrawable {
    pub index: Option<usize>,
    pub message: Option<Message>,
}

impl DialogDrawable {
    pub fn boxed() -> Box<dyn Drawable> {
        Box::new(Self::default())
    }
}

const BOX_HEIGHT: u32 = 128;
const H_PADDING: i32 = 16;
const V_PADDING: i32 = 16;
const LINE_SPACING: i32 = 4;

#[derive(Clone, Default, Debug)]
struct Line {
    height: u32,
    width: u32,
    segments: Vec<(String, Vec<Attribute>, Dimen)>,
}

impl Drawable for DialogDrawable {
    fn depth(&self) -> i32 {
        ::std::i32::MAX
    }

    fn render(&self, canvas: &mut dyn Canvas) -> engine::Result<()> {
        if let Some(message) = &self.message {
            // draw the dialog box
            let size = canvas.size();
            canvas.set_transform(Rect::from(Point::default(), size), Rect::from(Point::default(), size));
            canvas.set_color(Color::WHITE);
            canvas.draw_rect_filled(Rect::new(0, (size.height - BOX_HEIGHT) as i32, size.width, BOX_HEIGHT))?;
            canvas.set_color(Color::BLACK);
            canvas.draw_rect(Rect::new(0, (size.height - BOX_HEIGHT) as i32, size.width, 1))?;

            let max_width = size.width - 2 * H_PADDING as u32;

            // draw the text segments
            let segments =
                if let Some(index) = self.index {
                    message.message().up_to(index)
                } else {
                    message.message().clone()
                };

            // NOTE: might be worth storing the Vec<Line> until the message changes, and figuring
            // out the substring of that instead of finding a substring of the raw message and
            // recalculating positions every frame.

            let lines = segments.0
                .iter()
                .flat_map(|(text, attributes)|
                    text.split("\n")
                        .map(move |line| (line.to_owned(), attributes.clone()))
                )
                .fold(Ok(vec![Line::default()]), |lines: engine::Result<Vec<Line>>, (text, attributes)| {
                    let mut lines = lines?;
                    canvas.set_font(DEFAULT_FONT);
                    for attribute in &attributes {
                        match attribute {
                            &Attribute::Font(font) => canvas.set_font(*font),
                            _ => {},
                        }
                    }
                    let Dimen { width, height } = canvas.measure_text(text.to_owned())?;
                    lines.last_mut().unwrap().height = u32::max(lines.last().unwrap().height, height);
                    if lines.last().unwrap().width + width > max_width {
                        let len = text.len();
                        // TODO: could probably binary search here if it's too slow
                        let mut start = 0;
                        let mut end = 0;
                        loop {
                            let Dimen { width, height } = canvas.measure_text(text[start..end + 1].to_owned())?;
                            if lines.last().unwrap().width + width <= max_width {
                                end += 1;
                                if end != len - 1 {
                                    continue;
                                }
                            }
                            for current_line in lines.last_mut() {
                                current_line.width += width;
                                current_line.segments.push((text[start..end].to_owned(), attributes.clone(), Dimen { width, height }));
                            }
                            lines.push(Line::default());
                            if end == len - 1 {
                                break;
                            }
                            start = end;
                        }
                    } else {
                        let current_line = lines.last_mut().unwrap();
                        current_line.width += width;
                        current_line.segments.push((text, attributes, Dimen { width, height }));
                    }
                    Ok(lines)
                })?;
            let mut y = (size.height - BOX_HEIGHT) as i32 + V_PADDING;
            for line in lines {
                let mut x = H_PADDING;
                // TODO: this probably doesn't have so good baseline alignment
                for (text, attributes, Dimen { width, height }) in line.segments {
                    canvas.set_font(DEFAULT_FONT);
                    canvas.set_color(Color::BLACK);
                    for attribute in attributes {
                        match attribute {
                            Attribute::Color(color) => canvas.set_color(color),
                            Attribute::Font(font) => canvas.set_font(*font),
                        }
                    }
                    canvas.draw_text(Point::new(x, y + line.height as i32 - height as i32), text)?;
                    x += width as i32;
                }
                y += line.height as i32 + LINE_SPACING;
            }
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
