use std::any::Any;
use game_engine::prelude::*;
use inkgen::runtime::Paragraph;

use crate::model::{
    message::Message,
    pretty_string::Attributes,
};
use crate::font::default::REGULAR_20 as DEFAULT_FONT;

#[derive(Default, Debug)]
pub struct DialogDrawable {
    pub index: Option<usize>,
    pub paragraph: Option<Paragraph>,
    pub selection: usize,
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
    attributes: Attributes,
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
static mut PREVIOUS_MESSAGE: Option<Paragraph> = None;

impl Drawable for DialogDrawable {
    fn depth(&self) -> i32 {
        ::std::i32::MAX - 1
    }

    fn render(&self, canvas: &mut dyn Canvas) -> game_engine::Result<()> {
        if let Some(paragraph) = &self.paragraph {
            let message = Message::from(paragraph.text());

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
            let segments = message.message().clone().0;

            unsafe {
                if CALCULATED_LINES.is_none() || PREVIOUS_MESSAGE.as_ref() != Some(paragraph) {
                    PREVIOUS_MESSAGE = Some(paragraph.clone());
                    CALCULATED_LINES = Some(segments
                        .iter()
                        .flat_map(|(text, attributes)| {
                            let lines: Vec<_> = text.split("\n").collect();
                            let len = lines.len();
                            lines
                                .iter()
                                .enumerate()
                                .map(move |(i, line)| (line.to_string(), attributes, i != len - 1))
                                .collect::<Vec<_>>()
                        })
                        .fold(Ok(vec![Line::default()]), |lines: game_engine::Result<Vec<Line>>, (text, attributes, newline)| {
                            let mut lines = lines?;
                            if let Some(font) = attributes.font {
                                canvas.set_font(*font);
                            } else {
                                canvas.set_font(DEFAULT_FONT);
                            }
                            let Dimen { width, height } = canvas.measure_text(text.to_owned())?;
                            if lines.last().unwrap().width + width > max_width {
                                let len = text.len();
                                // TODO: could probably binary search here if it's too slow
                                let mut start = 0;
                                let mut end = 0;
                                loop {
                                    let mut word_len = 0;
                                    while let Some(ch) = text.chars().nth(end + word_len) {
                                        if ch.is_whitespace() {
                                            if word_len == 0 {
                                                end += 1;
                                            } else {
                                                break;
                                            }
                                        } else {
                                            word_len += 1;
                                        }
                                    }
                                    let Dimen { width, height } = canvas.measure_text(text.chars().skip(start).take(end + word_len - start).collect())?;
                                    if lines.last().unwrap().width + width <= max_width {
                                        end += word_len;
                                        if end < len - 1 {
                                            continue;
                                        }
                                    }
                                    let current_line = lines.last_mut().unwrap();
                                    let (min_y, max_y) = text.chars()
                                        .skip(start)
                                        .take(end - start)
                                        .filter_map(|ch| canvas.glyph_metrics(ch).ok().and_then(|x| x))
                                        .fold((None, None), |(min_y, max_y), metrics| (
                                            min_y.map(|min_y| i32::min(min_y, metrics.miny)).or(Some(metrics.miny)),
                                            max_y.map(|max_y| i32::max(max_y, metrics.maxy)).or(Some(metrics.maxy)),
                                        ));
                                    current_line.width += width;
                                    current_line.spacing = i32::max(current_line.spacing, canvas.line_spacing()?);
                                    current_line.height = u32::max(current_line.height, height);
                                    current_line.segments.push(Segment {
                                        text: text.chars()
                                            .skip(start)
                                            .take(end - start)
                                            .collect(),
                                        attributes: *attributes,
                                        size: Dimen { width, height },
                                        ascent: canvas.font_ascent()?,
                                        min_y: min_y.unwrap_or(0),
                                        max_y: max_y.unwrap_or(0),
                                    });
                                    lines.push(Line::default());
                                    if end >= len - 1 {
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
                                    attributes: *attributes,
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
                    for &Segment { ref text, attributes, size: Dimen { width, .. }, ascent, max_y, .. } in &line.segments {
                        if text.is_empty() { break; }
                        let to_print =
                            if self.index.is_some() && printed + text.len() > self.index.unwrap() {
                                text.chars().take(self.index.unwrap() - printed).collect()
                            } else {
                                text.to_owned()
                            };
                        printed += to_print.len();
                        canvas.set_font(DEFAULT_FONT);
                        canvas.set_color(Color::BLACK);
                        if let Some(color) = attributes.color {
                            canvas.set_color(color);
                        }
                        if let Some(font) = attributes.font {
                            canvas.set_font(*font);
                        }
                        canvas.draw_text(Point::new(x, y + ascent - max_y), to_print)?;
                        x += width as i32;
                        if self.index.is_some() && printed == self.index.unwrap() {
                            break 'done;
                        }
                    }
                    y += line.spacing;
                }
                if self.index.is_none() {
                    if let Some(choices) = paragraph.choices() {
                        canvas.set_font(DEFAULT_FONT);
                        let line_spacing = canvas.line_spacing()?;
                        let strings: Vec<_> = choices.iter().map(|choice| Message::from(choice).message().plain()).collect();

                        // TODO: arr_width is a constant... so measure it once manually, not every frame
                        let Dimen { width: arr_width, .. } = canvas.measure_text(String::from("=>"))?;

                        let bounds = strings.iter()
                            .try_fold(Dimen::default(), |size, string| -> game_engine::Result<Dimen> {
                                let Dimen { width, .. } = canvas.measure_text(format!("{}", string))?;
                                Ok(Dimen::new(
                                    u32::max(width, size.width),
                                    size.height + line_spacing as u32,
                                ))
                            })?
                            .extend(Dimen::new(32 + arr_width, 16));
                        let options_box = Rect::from(
                            Point::new(
                                (size.width - bounds.width) as i32,
                                (size.height - BOX_HEIGHT - bounds.height) as i32,
                            ),
                            bounds,
                        );
                        canvas.set_color(Color::WHITE);
                        canvas.draw_rect_filled(options_box)?;
                        canvas.set_color(Color::BLACK);
                        for (i, message) in strings.into_iter().enumerate() {
                            let point = Point::new(
                                options_box.x + 16 + arr_width as i32,
                                options_box.y + 8 + line_spacing * i as i32,
                            );
                            canvas.draw_text(point, message)?;
                            if i == self.selection {
                                let point = Point::new(
                                    options_box.x + 8 as i32,
                                    options_box.y + 8 + line_spacing * i as i32,
                                );
                                canvas.draw_text(point, String::from("=>"))?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
