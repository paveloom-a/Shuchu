use fltk::{
    app,
    button::Button,
    enums::{Event, FrameType, Key, Shortcut},
    group::{Pack, PackType},
    image::SvgImage,
    prelude::*,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::app::CONSTANTS;
use crate::ui::logic;

pub fn menubar(channels: &Channels) -> Pack {
    let mut menubar = Pack::default().with_size(0, CONSTANTS.rewards_menubar_height);
    menubar.set_type(PackType::Horizontal);

    let mut add_image = SvgImage::from_data(include_str!("../../../assets/menu/plus.svg")).unwrap();
    add_image.scale(24, 24, true, true);

    let mut add_button = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    add_button.set_image(Some(add_image));
    add_button.set_frame(FrameType::FlatBox);

    let mut delete_image =
        SvgImage::from_data(include_str!("../../../assets/menu/minus.svg")).unwrap();
    delete_image.scale(24, 24, true, true);

    let mut delete_button = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    delete_button.set_image(Some(delete_image));
    delete_button.set_frame(FrameType::FlatBox);

    menubar_logic(&mut menubar);
    add_button_logic(&mut add_button, channels);
    delete_button_logic(&mut delete_button);

    menubar.end();
    menubar
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

fn add_button_logic<T: ButtonExt + WidgetBase + 'static>(ab: &mut T, channels: &Channels) {
    ab.set_shortcut(Shortcut::from_char('a'));
    ab.set_callback({
        let s = channels.rewards_edit.s.clone();
        move |_| {
            s.try_send(events::ADD_A_REWARD_OPEN).ok();
        }
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
