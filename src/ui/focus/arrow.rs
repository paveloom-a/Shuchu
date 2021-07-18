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
    let mut a = Button::default();
    a.set_frame(FrameType::FlatBox);
    a.set_tooltip("Show the Conversion Rates pane");

    draw_frame(&mut a);
    a.draw(draw_frame);

    let mut ai = SvgImage::from_data(include_str!("../../../assets/menu/arrow.svg")).unwrap();
    ai.scale(48, 40, true, true);
    a.set_image(Some(ai));

    logic(&mut a);

    a
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
            if let Some(ref mut c) = p.child(1) {
                if let Some(ref mut w) = p.parent() {
                    if let Some(ref mut re_p) = w.child(1) {
                        if let Some(ref mut ra_p) = w.child(2) {
                            if re_p.visible() {
                                re_p.hide();
                                ra_p.show();
                                a.set_tooltip("Hide the Conversion Rates pane");
                            } else if ra_p.visible() {
                                w.resize(
                                    w.x(),
                                    w.y(),
                                    w.w(),
                                    10 + CONSTANTS.focus_pane_height + 10,
                                );
                                ra_p.hide();
                                a.set_tooltip("Show the Conversion Rates pane");
                            } else {
                                w.resize(w.x(), w.y(), w.w(), CONSTANTS.main_window_height);
                                ra_p.show();
                                a.set_tooltip("Hide the Conversion Rates pane");
                            }
                            c.set_tooltip("Show the Rewards pane");
                        }
                    }
                }
            }
        }
    });
    a.handle({
        let unselect_box = FrameType::FlatBox;
        move |a, ev| match ev {
            Event::KeyDown => match app::event_key() {
                Key::Left => logic::fp_handle_left(a, 2),
                Key::Right => logic::fp_handle_right(a, 2),
                Key::Tab => logic::handle_tab(a),
                _ => logic::handle_selection(a, ev, unselect_box),
            },
            _ => logic::handle_selection(a, ev, unselect_box),
        }
    });
}
