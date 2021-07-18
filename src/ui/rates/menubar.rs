use fltk::{
    app,
    button::Button,
    enums::{Event, FrameType, Key, Shortcut},
    group::{Pack, PackType},
    image::SvgImage,
    prelude::*,
};

use crate::ui::app::CONSTANTS;
use crate::ui::logic;

pub fn menubar() -> Pack {
    let mut m = Pack::default().with_size(0, CONSTANTS.rewards_menubar_height);
    m.set_type(PackType::Horizontal);

    // 1. Add a Rate
    let _ab = add_button();

    // 2. Delete the Rate
    let _db = delete_button();

    m.end();

    menubar_logic(&mut m);

    m
}

fn menubar_logic<T: WidgetBase>(m: &mut T) {
    m.handle(|m, ev| match ev {
        Event::KeyDown => {
            if app::event_key() == Key::Tab {
                if let Some(ref p) = m.parent() {
                    if let Some(ref mut l) = p.child(1) {
                        l.take_focus().ok();
                    }
                }
                true
            } else {
                false
            }
        }
        _ => false,
    });
}

fn add_button() -> Button {
    let mut ai = SvgImage::from_data(include_str!("../../../assets/menu/plus.svg")).unwrap();
    ai.scale(24, 24, true, true);

    let mut ab = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    ab.set_image(Some(ai));
    ab.set_frame(FrameType::FlatBox);
    ab.set_tooltip("Add a Rate");

    add_button_logic(&mut ab);

    ab
}

fn add_button_logic<T: ButtonExt + WidgetBase + 'static>(ab: &mut T) {
    ab.set_shortcut(Shortcut::from_char('a'));
    ab.set_callback(|_| {
        println!("Add Pressed!");
    });
    ab.handle(|ab, ev| {
        if ev == Event::KeyDown {
            match app::event_key() {
                Key::Left => logic::rp_handle_left(ab, 0),
                Key::Right => logic::rp_handle_right(ab, 0),
                _ => logic::handle_selection(ab, ev, FrameType::FlatBox),
            }
        } else {
            logic::handle_selection(ab, ev, FrameType::FlatBox)
        }
    });
}

fn delete_button() -> Button {
    let mut di = SvgImage::from_data(include_str!("../../../assets/menu/minus.svg")).unwrap();
    di.scale(24, 24, true, true);

    let mut db = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    db.set_image(Some(di));
    db.set_frame(FrameType::FlatBox);
    db.set_tooltip("Delete the Rate");

    delete_button_logic(&mut db);

    db
}

fn delete_button_logic<T: ButtonExt + WidgetBase + 'static>(db: &mut T) {
    db.set_shortcut(Shortcut::from_char('d'));
    db.set_callback(|_| {
        println!("Delete Pressed!");
    });
    db.handle(|db, ev| {
        if ev == Event::KeyDown {
            match app::event_key() {
                Key::Left => logic::rp_handle_left(db, 1),
                Key::Right => logic::rp_handle_right(db, 1),
                _ => logic::handle_selection(db, ev, FrameType::FlatBox),
            }
        } else {
            logic::handle_selection(db, ev, FrameType::FlatBox)
        }
    });
}
