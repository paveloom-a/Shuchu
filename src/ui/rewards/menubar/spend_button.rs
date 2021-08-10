//! Spend Button deletes the selected item and subtracts its price from the total number of
//! coins.

use fltk::{app, button::Button, image::SvgImage, prelude::*};

use crate::events;
use crate::ui::constants::{
    DOWN_BUTTON_FRAME_TYPE, FLAT_BUTTON_FRAME_TYPE, REWARDS_MENUBAR_HEIGHT,
};
use crate::ui::logic;

/// Initialize the spend button
pub fn spend_button() -> Button {
    let mut si =
        SvgImage::from_data(include_str!("../../../../assets/menu/insert-coin.svg")).unwrap();
    si.scale(24, 24, true, true);

    let mut sb = Button::default().with_size(REWARDS_MENUBAR_HEIGHT, 0);
    sb.set_image(Some(si));
    sb.set_frame(FLAT_BUTTON_FRAME_TYPE);
    sb.set_down_frame(DOWN_BUTTON_FRAME_TYPE);
    sb.set_tooltip("Spend Coins for the Reward");

    logic(&mut sb);

    sb
}

/// Set a callback and a handler for the spend button
fn logic<T: ButtonExt + WidgetBase + 'static>(sb: &mut T) {
    // The callback is disabled by default since there
    // is no reward selection at the start of the program
    sb.set_callback(|_| {});
    // Handle the shortcut, lock, and focus / selection events
    sb.handle({
        // This button is locked by default. Selecting an item will unlock it
        let mut lock = true;
        move |sb, ev| {
            let bits = ev.bits();
            logic::handle_shortcut(sb, bits, events::SPEND_BUTTON_SHORTCUT)
                || logic::handle_lock(
                    sb,
                    &mut lock,
                    callback,
                    bits,
                    events::LOCK_THE_SPEND_BUTTON,
                    events::UNLOCK_THE_SPEND_BUTTON,
                )
                || logic::handle_button(sb, ev, 4, lock)
        }
    });
}

/// The callback of the spend button when it's active
fn callback<T: WidgetBase>(_: &mut T) {
    // Start a chain of events to spend coins on the selected reward (if it's affordable)
    app::handle_main(events::SPEND_COINS_SEND_PRICE).ok();
}
