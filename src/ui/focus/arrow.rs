use fltk::{
    app,
    button::Button,
    enums::{Event, FrameType, Key, Shortcut},
    image::SvgImage,
    prelude::*,
};

use crate::ui::app::CONSTANTS;
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

    logic(&mut arrow);

    arrow
}

fn draw_frame<T: WidgetExt>(f: &mut T) {
    if let Some(ref p) = f.parent() {
        if let Some(ref lw) = p.child(0) {
            if let Some(ref rw) = p.child(1) {
                let lx = lw.x() + lw.w();
                let w = 48 + 15;
                f.set_pos(lx + (rw.x() - lx - w) / 2, p.y() + 15);
                f.set_size(w, 30);
            }
        }
    }
}

fn logic<T: WidgetBase + ButtonExt>(a: &mut T) {
    a.set_shortcut(Shortcut::from_char('r'));
    a.set_callback(|a| {
        if let Some(ref p) = a.parent() {
            if let Some(ref mut w) = p.parent() {
                if let Some(ref mut re_p) = w.child(1) {
                    if let Some(ref mut ra_p) = w.child(2) {
                        if re_p.visible() {
                            re_p.hide();
                            ra_p.show();
                        } else if ra_p.visible() {
                            w.resize(w.x(), w.y(), w.w(), 10 + CONSTANTS.focus_pane_height + 10);
                            ra_p.hide();
                        } else {
                            w.resize(w.x(), w.y(), w.w(), CONSTANTS.main_window_height);
                            ra_p.show();
                        }
                    }
                }
            }
        }
    });
    a.handle(|a, ev| {
        if ev == Event::KeyDown {
            match app::event_key() {
                Key::Left => logic::fp_handle_left(a, 2),
                Key::Right => logic::fp_handle_right(a, 2),
                Key::Tab => logic::handle_tab(a),
                _ => logic::handle_selection(a, ev, FrameType::FlatBox),
            }
        } else {
            logic::handle_selection(a, ev, FrameType::FlatBox)
        }
    });
}
