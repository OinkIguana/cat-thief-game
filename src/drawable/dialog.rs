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

#[derive(Clone, Debug)]
struct Segment {
    text: String,
    attributes: Vec<Attribute>,
    size: Dimen,
    ascent: i32,
    min_y: i32,
    max_y: i32,
}

#[derive(Clone, Default, Debug)]
struct Line {
    height: u32,
    width: u32,
    spacing: i32,
    segments: Vec<Segment>,
}

// This will be ok because there should only ever be ONE DialogDrawable at one time, and even if
// there are multiple of them, Drawables are all handled sequentially, so they should not cause
// simultaneous access here.
static mut CALCULATED_LINES: Option<Vec<Line>> = None;
static mut PREVIOUS_MESSAGE: Option<Message> = None;

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
            canvas.set_font(DEFAULT_FONT);
            canvas.draw_rect_filled(Rect::new(0, (size.height - BOX_HEIGHT) as i32, size.width, BOX_HEIGHT))?;
            canvas.set_color(Color::BLACK);
            canvas.draw_rect(Rect::new(0, (size.height - BOX_HEIGHT) as i32, size.width, 1))?;

            if let Some(speaker) = message.speaker().to_owned() {
                let Dimen { width, height } = canvas.measure_text(speaker.clone())?;
                canvas.set_color(Color::WHITE);
                let speaker_box = Rect::new(
                    H_PADDING,
                    (size.height as i32 - BOX_HEIGHT as i32 - 2 * V_PADDING - height as i32) as i32,
                    width + 2 * H_PADDING as u32,
                    2 * V_PADDING as u32 + height,
                );
                canvas.draw_rect_filled(speaker_box)?;
                canvas.set_color(Color::BLACK);
                canvas.draw_rect(speaker_box)?;
                canvas.draw_text(Point::new(speaker_box.x + H_PADDING, speaker_box.y + V_PADDING), speaker)?;
            }

            let max_width = size.width - 2 * H_PADDING as u32;

            // draw the text segments
            let segments = message.message().clone();

            unsafe {
                if CALCULATED_LINES.is_none() || PREVIOUS_MESSAGE.as_ref() != Some(message) {
                    PREVIOUS_MESSAGE = Some(message.clone());
                    CALCULATED_LINES = Some(segments.0
                        .iter()
                        .flat_map(|(text, attributes)| {
                            let lines: Vec<_> = text.split("\n").collect();
                            let len = lines.len();
                            lines
                                .iter()
                                .enumerate()
                                .map(move |(i, line)| (line.to_string(), attributes.clone(), i != len - 1))
                                .collect::<Vec<_>>()
                        })
                        .fold(Ok(vec![Line::default()]), |lines: engine::Result<Vec<Line>>, (text, attributes, newline)| {
                            let mut lines = lines?;
                            canvas.set_font(DEFAULT_FONT);
                            for attribute in &attributes {
                                match attribute {
                                    &Attribute::Font(font) => canvas.set_font(*font),
                                    _ => {},
                                }
                            }
                            let Dimen { width, height } = canvas.measure_text(text.to_owned())?;
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
                                    if let Some(current_line) = lines.last_mut() {
                                        let substr = &text[start..end];
                                        let (min_y, max_y) = substr.chars()
                                            .filter_map(|ch| canvas.glyph_metrics(ch).ok().and_then(|x| x))
                                            .fold((None, None), |(min_y, max_y), metrics| (
                                                min_y.map(|min_y| i32::min(min_y, metrics.miny)).or(Some(metrics.miny)),
                                                max_y.map(|max_y| i32::max(max_y, metrics.maxy)).or(Some(metrics.maxy)),
                                            ));
                                        current_line.width += width;
                                        current_line.spacing = i32::max(current_line.spacing, canvas.line_spacing()?);
                                        current_line.height = u32::max(current_line.height, height);
                                        current_line.segments.push(Segment {
                                            text: substr.to_string(),
                                            attributes: attributes.clone(),
                                            size: Dimen { width, height },
                                            ascent: canvas.font_ascent()?,
                                            min_y: min_y.unwrap_or(0),
                                            max_y: max_y.unwrap_or(0),
                                        });
                                    }
                                    lines.push(Line::default());
                                    if end == len - 1 {
                                        break;
                                    }
                                    start = end;
                                }
                            } else {
                                let current_line = lines.last_mut().unwrap();
                                let (min_y, max_y) = text.chars()
                                    .filter_map(|ch| canvas.glyph_metrics(ch).ok().and_then(|x| x))
                                    .fold((None, None), |(min_y, max_y), metrics| (
                                        min_y.map(|min_y| i32::min(min_y, metrics.miny)).or(Some(metrics.miny)),
                                        max_y.map(|max_y| i32::max(max_y, metrics.maxy)).or(Some(metrics.maxy)),
                                    ));

                                current_line.width += width;
                                current_line.spacing = i32::max(current_line.spacing, canvas.line_spacing()?);
                                current_line.height = u32::max(current_line.height, height);
                                current_line.segments.push(Segment {
                                    text,
                                    attributes,
                                    size: Dimen { width, height },
                                    ascent: canvas.font_ascent()?,
                                    min_y: min_y.unwrap_or(0),
                                    max_y: max_y.unwrap_or(0),
                                });
                            }
                            if newline {
                                lines.push(Line::default());
                            }
                            Ok(lines)
                        })?
                    );
                }
                let lines = CALCULATED_LINES.as_ref().unwrap();
                let mut y = (size.height - BOX_HEIGHT) as i32 + V_PADDING;
                let mut printed = 0usize;
                'done: for line in lines {
                    let mut x = H_PADDING;
                    for &Segment { ref text, ref attributes, size: Dimen { width, .. }, ascent, max_y, .. } in &line.segments {
                        if text.is_empty() { break; }
                        let to_print =
                            if self.index.is_some() && printed + text.len() > self.index.unwrap() {
                                &text[..self.index.unwrap() - printed]
                            } else {
                                &text
                            };
                        printed += to_print.len();
                        canvas.set_font(DEFAULT_FONT);
                        canvas.set_color(Color::BLACK);
                        for attribute in attributes {
                            match attribute {
                                &Attribute::Color(color) => canvas.set_color(color),
                                &Attribute::Font(font) => canvas.set_font(*font),
                            }
                        }
                        canvas.draw_text(Point::new(x, y + ascent - max_y), to_print.to_string())?;
                        x += width as i32;
                        if self.index.is_some() && printed == self.index.unwrap() {
                            break 'done;
                        }
                    }
                    y += line.spacing;
                }
            }
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
