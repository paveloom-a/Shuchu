//! OK button combines the values of the valuators in an item and sends this item to the
//! [Rewards](super)' list.
//!
//! The OK button's callback changes, depending on whether the user wants to add a new reward or
//! edit the existing one.

use fltk::{app, button::Button, prelude::*};

use crate::events;
use crate::ui::constants::{BOXED_BUTTON_FRAME_TYPE, DOWN_BUTTON_FRAME_TYPE};
use crate::ui::logic;

/// Initialize the OK button
pub fn ok() -> Button {
    let mut ok = Button::default().with_size(80, 0).with_label("OK");
    ok.set_frame(BOXED_BUTTON_FRAME_TYPE);
    ok.set_down_frame(DOWN_BUTTON_FRAME_TYPE);

    logic(&mut ok);

    ok
}

/// Set a callback and a handler for the OK button
fn logic<T: WidgetBase + ButtonExt + 'static>(b: &mut T) {
    // As a safety measure, the default callback is set to add a reward
    b.set_callback(add_callback);
    // Handle the custom events
    b.handle(|b, ev|
    // In case of the
    match ev.bits() {
        // 'Do callback' event
        events::OK_BUTTON_DO_CALLBACK => {
            b.do_callback();
            true
        }
        // 'Set to add` event
        events::OK_BUTTON_SET_TO_ADD => {
            // Set the callback to add a reward
            b.set_callback(add_callback);
            true
        }
        // 'Set to edit` event
        events::OK_BUTTON_SET_TO_EDIT => {
            // Set the callback to edit the reward
            b.set_callback(edit_callback);
            true
        }
        // Otherwise, handle the focus / selection events
        _ => logic::handle_selection(b, ev, BOXED_BUTTON_FRAME_TYPE, false),
    });
}

/// 'Add a reward' callback
fn add_callback<T: WidgetBase>(_: &mut T) {
    app::handle_main(events::ADD_A_REWARD_SEND_COINS).ok();
}

/// 'Edit the reward' callback
fn edit_callback<T: WidgetBase>(_: &mut T) {
    app::handle_main(events::EDIT_THE_REWARD_SEND_COINS).ok();
}
