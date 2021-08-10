//! The edit button opens the [Rewards Edit](super::super::edit)
//! window, set to edit the selected item.

use fltk::{app, button::Button, image::SvgImage, prelude::*};

use crate::events;
use crate::ui::constants::{
    DOWN_BUTTON_FRAME_TYPE, FLAT_BUTTON_FRAME_TYPE, REWARDS_MENUBAR_HEIGHT,
};
use crate::ui::logic;

/// Initialize the edit button
pub fn edit_button() -> Button {
    let mut ei = SvgImage::from_data(include_str!("../../../../assets/menu/pencil.svg")).unwrap();
    ei.scale(24, 24, true, true);

    let mut eb = Button::default().with_size(REWARDS_MENUBAR_HEIGHT, 0);
    eb.set_image(Some(ei));
    eb.set_frame(FLAT_BUTTON_FRAME_TYPE);
    eb.set_down_frame(DOWN_BUTTON_FRAME_TYPE);
    eb.set_tooltip("Edit the Reward");

    logic(&mut eb);

    eb
}

/// Set a callback and a handler for the edit button
fn logic<T: ButtonExt + WidgetBase + 'static>(eb: &mut T) {
    // The callback is disabled by default since there
    // is no reward selection at the start of the program
    eb.set_callback(|_| {});
    // Handle the shortcut, lock, and focus / selection events
    eb.handle({
        // This button is locked by default. Selecting an item will unlock it
        let mut lock = true;
        move |eb, ev| {
            let bits = ev.bits();
            logic::handle_shortcut(eb, bits, events::EDIT_BUTTON_SHORTCUT)
                || logic::handle_lock(
                    eb,
                    &mut lock,
                    callback,
                    bits,
                    events::LOCK_THE_EDIT_A_REWARD_BUTTON,
                    events::UNLOCK_THE_EDIT_A_REWARD_BUTTON,
                )
                || logic::handle_button(eb, ev, 2, lock)
        }
    });
}

/// The callback of the edit button when it's active
fn callback<T: WidgetBase>(_: &mut T) {
    // Start a chain of events to edit the selected reward
    app::handle_main(events::EDIT_THE_REWARD_SEND_ITEM).ok();
}
