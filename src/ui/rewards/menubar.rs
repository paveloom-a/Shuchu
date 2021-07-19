use fltk::{
    app,
    button::Button,
    enums::{Event, FrameType, Key, Shortcut},
    frame::Frame,
    group::{Pack, PackType},
    image::SvgImage,
    prelude::*,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::app::CONSTANTS;
use crate::ui::logic;

pub fn menubar(channels: &Channels) -> Pack {
    let mut m = Pack::default().with_size(0, CONSTANTS.rewards_menubar_height);
    m.set_type(PackType::Horizontal);

    // 1. Add a Reward
    let _ab = add_button(channels);

    // 2. Delete the Reward
    let _db = delete_button();

    // 3. Edit the Reward
    let _eb = edit_button();

    // 4. Divider
    let _d = divider();

    // 5. Spend Coins for the Reward
    let _sb = spend_button();

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

fn add_button(channels: &Channels) -> Button {
    let mut ai = SvgImage::from_data(include_str!("../../../assets/menu/plus.svg")).unwrap();
    ai.scale(24, 24, true, true);

    let mut ab = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    ab.set_image(Some(ai));
    ab.set_frame(FrameType::FlatBox);
    ab.set_tooltip("Add a Reward");

    add_button_logic(&mut ab, channels);

    ab
}

fn add_button_logic<T: ButtonExt + WidgetBase + 'static>(ab: &mut T, channels: &Channels) {
    ab.set_shortcut(Shortcut::from_char('a'));
    ab.set_callback({
        let s = channels.rewards_edit.s.clone();
        move |_| {
            s.try_send(events::ADD_A_REWARD_OPEN).ok();
            s.try_send(events::OK_BUTTON_SET_TO_ADD).ok();
        }
    });
    ab.handle(|ab, ev| match ev {
        Event::KeyDown => match app::event_key() {
            Key::Left => logic::rp_handle_left(ab, 0),
            Key::Right => logic::rp_handle_right(ab, 0),
            _ => logic::handle_selection(ab, ev, FrameType::FlatBox),
        },
        _ => logic::handle_selection(ab, ev, FrameType::FlatBox),
    });
}

fn delete_button() -> Button {
    let mut di = SvgImage::from_data(include_str!("../../../assets/menu/minus.svg")).unwrap();
    di.scale(24, 24, true, true);

    let mut db = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    db.set_image(Some(di));
    db.set_frame(FrameType::FlatBox);
    db.set_tooltip("Delete the Reward");

    delete_button_logic(&mut db);

    db
}

fn delete_button_logic<T: ButtonExt + WidgetBase + 'static>(db: &mut T) {
    db.set_shortcut(Shortcut::from_char('d'));
    db.set_callback(|_| {
        app::handle_main(events::DELETE_A_REWARD).ok();
    });
    db.handle(|db, ev| match ev {
        Event::KeyDown => match app::event_key() {
            Key::Left => logic::rp_handle_left(db, 1),
            Key::Right => logic::rp_handle_right(db, 1),
            _ => logic::handle_selection(db, ev, FrameType::FlatBox),
        },
        _ => logic::handle_selection(db, ev, FrameType::FlatBox),
    });
}

fn edit_button() -> Button {
    let mut ei = SvgImage::from_data(include_str!("../../../assets/menu/pencil.svg")).unwrap();
    ei.scale(24, 24, true, true);

    let mut eb = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    eb.set_image(Some(ei));
    eb.set_frame(FrameType::FlatBox);
    eb.set_tooltip("Edit the Reward");

    edit_button_logic(&mut eb);

    eb
}

fn edit_button_logic<T: ButtonExt + WidgetBase + 'static>(eb: &mut T) {
    eb.set_shortcut(Shortcut::from_char('e'));
    eb.set_callback({
        move |_| {
            app::handle_main(events::EDIT_A_REWARD_SEND_ITEM).ok();
        }
    });
    eb.handle(|eb, ev| match ev {
        Event::KeyDown => match app::event_key() {
            Key::Left => logic::rp_handle_left(eb, 2),
            Key::Right => logic::rp_handle_right(eb, 2),
            _ => logic::handle_selection(eb, ev, FrameType::FlatBox),
        },
        _ => logic::handle_selection(eb, ev, FrameType::FlatBox),
    });
}

fn divider() -> Frame {
    let mut di = SvgImage::from_data(include_str!("../../../assets/menu/divider.svg")).unwrap();
    di.scale(8, 24, true, true);

    let mut d = Frame::default().with_size(10, 0);
    d.set_image(Some(di));

    d
}

fn spend_button() -> Button {
    let mut si = SvgImage::from_data(include_str!("../../../assets/menu/insert-coin.svg")).unwrap();
    si.scale(24, 24, true, true);

    let mut sb = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    sb.set_image(Some(si));
    sb.set_frame(FrameType::FlatBox);
    sb.set_tooltip("Spend Coins for the Reward");

    spend_button_logic(&mut sb);

    sb
}

fn spend_button_logic<T: ButtonExt + WidgetBase + 'static>(sb: &mut T) {
    sb.set_shortcut(Shortcut::from_char('s'));
    sb.set_callback(|_| {
        app::handle_main(events::SPEND_COINS_SEND_TOTAL).ok();
    });
    sb.handle(|sb, ev| match ev {
        Event::KeyDown => match app::event_key() {
            Key::Left => logic::rp_handle_left(sb, 2),
            Key::Right => logic::rp_handle_right(sb, 2),
            _ => logic::handle_selection(sb, ev, FrameType::FlatBox),
        },
        _ => logic::handle_selection(sb, ev, FrameType::FlatBox),
    });
}
