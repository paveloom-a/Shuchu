use fltk::{app, button::Button, enums::FrameType, prelude::*};

use crate::events;
use crate::ui::logic;

pub fn ok() -> Button {
    let mut ok = Button::default().with_size(80, 0).with_label("OK");

    logic(&mut ok);

    ok
}

fn logic<T: WidgetBase + ButtonExt + 'static>(b: &mut T) {
    b.set_callback(add_callback);
    b.handle(|b, ev| match ev.bits() {
        events::OK_BUTTON_SET_TO_ADD => {
            b.set_callback(add_callback);
            true
        }
        events::OK_BUTTON_SET_TO_EDIT => {
            b.set_callback(edit_callback);
            true
        }
        _ => logic::handle_selection(b, ev, FrameType::BorderBox),
    });
}

fn add_callback<T: WidgetBase>(_: &mut T) {
    app::handle_main(events::ADD_A_REWARD_SEND_COINS).ok();
}

fn edit_callback<T: WidgetBase>(_: &mut T) {
    app::handle_main(events::EDIT_A_REWARD_SEND_COINS).ok();
}
