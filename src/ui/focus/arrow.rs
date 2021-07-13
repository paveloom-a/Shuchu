use fltk::{
    button::Button,
    enums::{FrameType, Shortcut},
    image::SvgImage,
    prelude::*,
};

use crate::ui::logic;

pub fn arrow() -> Button {
    let mut arrow = Button::default();
    arrow.set_frame(FrameType::FlatBox);

    draw_frame(&mut arrow);
    arrow.draw(draw_frame);

    let mut arrow_image =
        SvgImage::from_data(include_str!("../../../assets/menu/arrow.svg")).unwrap();
    arrow_image.scale(48, 40, true, true);
    arrow.set_image(Some(arrow_image));

    arrow.set_shortcut(Shortcut::from_char('r'));
    arrow.set_callback(|_| println!("Rate pressed!"));
    arrow.handle(logic::button_handle);

    arrow
}

fn draw_frame<T: WidgetExt>(f: &mut T) {
    if let Some(p) = f.parent() {
        if let Some(lw) = p.child(0) {
            if let Some(rw) = p.child(1) {
                let lx = lw.x() + lw.w();
                let w = 48 + 15;
                f.set_pos(lx + (rw.x() - lx - w) / 2, p.y() + 15);
                f.set_size(w, 30);
            }
        }
    }
}
