use criterion::{black_box, criterion_group, criterion_main, Criterion};
use piet_common::{
    kurbo::{Rect, Size},
    Device, OverflowMethod, RenderContext, Text, TextLayoutBuilder,
};

static TEXT: &str = r#"Philosophers often behave like little children who scribble some marks on a piece of paper at random and then ask the grown-up "What's that?" — It happened like this: the grown-up had drawn pictures for the child several times and said "this is a man," "this is a house," etc. And then the child makes some marks too and asks: what's this then?"#;

pub fn bench_render_text(c: &mut Criterion) {
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

    c.bench_function("text_render_layout_build", |b| {
        let mut device = Device::new().unwrap();
        let mut target = device.bitmap_target(600, 200, 1.0).unwrap();
        let mut rc = target.render_context();
        b.iter(|| {
            for _ in 0..10 {
                let _default_text = rc
                    .text()
                    .new_text_layout(TEXT)
                    .max_width(default_text_rect.width())
                    .max_height(default_text_rect.height())
                    .overflow(OverflowMethod::Default)
                    .build()
                    .unwrap();
                let _clip_text = rc
                    .text()
                    .new_text_layout(TEXT)
                    .max_width(default_text_rect.width())
                    .max_height(default_text_rect.height())
                    .overflow(OverflowMethod::Clip)
                    .build()
                    .unwrap();
                let _ellipsis_text = rc
                    .text()
                    .new_text_layout(TEXT)
                    .max_width(default_text_rect.width())
                    .max_height(default_text_rect.height())
                    .overflow(OverflowMethod::Ellipsis)
                    .build()
                    .unwrap();
            }
        });
    });

    c.bench_function("text_render_layout_draw", |b| {
        let mut device = Device::new().unwrap();
        let mut target = device.bitmap_target(600, 200, 1.0).unwrap();
        let mut rc = target.render_context();

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

        b.iter(|| {
            for _ in 0..10 {
                rc.draw_text(&default_text, default_text_rect.origin());
                rc.draw_text(&clip_text, clip_text_rect.origin());
                rc.draw_text(&ellipsis_text, ellipsis_text_rect.origin());
            }
        });
    });
}

criterion_group!(benches, bench_render_text);
criterion_main!(benches);
