use fltk::{
    app,
    button::Button,
    enums::{Event, FrameType, Key, Shortcut},
    image::SvgImage,
    prelude::*,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::app::CONSTANTS;
use crate::ui::logic;

pub fn add_button(channels: &Channels) -> Button {
    let mut ai = SvgImage::from_data(include_str!("../../../../assets/menu/plus.svg")).unwrap();
    ai.scale(24, 24, true, true);

    let mut ab = Button::default().with_size(CONSTANTS.rewards_menubar_height, 0);
    ab.set_image(Some(ai));
    ab.set_frame(FrameType::FlatBox);
    ab.set_down_frame(FrameType::DownBox);
    ab.set_tooltip("Add a Reward");

    logic(&mut ab, channels);

    ab
}

fn logic<T: ButtonExt + WidgetBase + 'static>(ab: &mut T, channels: &Channels) {
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
            _ => logic::handle_active_selection(ab, ev, FrameType::FlatBox),
        },
        _ => logic::handle_active_selection(ab, ev, FrameType::FlatBox),
    });
}
