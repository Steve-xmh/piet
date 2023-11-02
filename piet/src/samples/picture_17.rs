//! text_overflow_test
//!
//! This tests overflow methods of texts.

use crate::{
    kurbo::{Rect, Size},
    OverflowMethod, Text, TextLayoutBuilder,
};
use crate::{Color, Error, InterpolationMode, RenderContext};

pub const SIZE: Size = Size::new(600., 200.);

const RED: Color = Color::rgb8(255, 0, 0);
const BLUE: Color = Color::rgb8(0, 0, 255);
const INTERPOLATION_MODE: InterpolationMode = InterpolationMode::NearestNeighbor;
const BORDER_WIDTH: f64 = 4.0;

static TEXT: &str = r#"Philosophers often behave like little children who scribble some marks on a piece of paper at random and then ask the grown-up "What's that?" — It happened like this: the grown-up had drawn pictures for the child several times and said "this is a man," "this is a house," etc. And then the child makes some marks too and asks: what's this then?"#;

pub fn draw<R: RenderContext>(rc: &mut R) -> Result<(), Error> {
    rc.clear(None, Color::WHITE);

    let default_text_rect = Rect::from_origin_size((10., 40.), (180., 150.));
    let clip_text_rect = Rect::from_origin_size(
        (
            default_text_rect.max_x() + 20.,
            default_text_rect.origin().y,
        ),
        default_text_rect.size(),
    );
    let ellipsis_text_rect = Rect::from_origin_size(
        (clip_text_rect.max_x() + 20., clip_text_rect.origin().y),
        clip_text_rect.size(),
    );

    let default_text_label = rc
        .text()
        .new_text_layout("OverflowMethod::Default")
        .build()
        .unwrap();
    let clip_text_label = rc
        .text()
        .new_text_layout("OverflowMethod::Clip")
        .build()
        .unwrap();
    let ellipsis_text_label = rc
        .text()
        .new_text_layout("OverflowMethod::Ellipsis")
        .build()
        .unwrap();

    rc.draw_text(&default_text_label, default_text_rect.origin() - (0., 25.));
    rc.draw_text(&clip_text_label, clip_text_rect.origin() - (0., 25.));
    rc.draw_text(
        &ellipsis_text_label,
        ellipsis_text_rect.origin() - (0., 25.),
    );

    let default_text = rc
        .text()
        .new_text_layout(TEXT)
        .max_width(default_text_rect.width())
        .max_height(default_text_rect.height())
        .overflow(OverflowMethod::Default)
        .build()
        .unwrap();
    let clip_text = rc
        .text()
        .new_text_layout(TEXT)
        .max_width(default_text_rect.width())
        .max_height(default_text_rect.height())
        .overflow(OverflowMethod::Clip)
        .build()
        .unwrap();
    let ellipsis_text = rc
        .text()
        .new_text_layout(TEXT)
        .max_width(default_text_rect.width())
        .max_height(default_text_rect.height())
        .overflow(OverflowMethod::Ellipsis)
        .build()
        .unwrap();

    rc.draw_text(&default_text, default_text_rect.origin());
    rc.draw_text(&clip_text, clip_text_rect.origin());
    rc.draw_text(&ellipsis_text, ellipsis_text_rect.origin());

    rc.stroke(default_text_rect, &BLUE, 1.);
    rc.stroke(clip_text_rect, &BLUE, 1.);
    rc.stroke(ellipsis_text_rect, &BLUE, 1.);

    Ok(())
}
