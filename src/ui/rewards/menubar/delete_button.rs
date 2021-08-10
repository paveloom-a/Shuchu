//! Delete Button deletes the selected item in the [Rewards](super::super)' List.

use fltk::{app, button::Button, image::SvgImage, prelude::*};

use crate::events;
use crate::ui::constants::{
    DOWN_BUTTON_FRAME_TYPE, FLAT_BUTTON_FRAME_TYPE, REWARDS_MENUBAR_HEIGHT,
};
use crate::ui::logic;

/// Initialize the delete button
pub fn delete_button() -> Button {
    let mut di = SvgImage::from_data(include_str!("../../../../assets/menu/minus.svg")).unwrap();
    di.scale(24, 24, true, true);

    let mut db = Button::default().with_size(REWARDS_MENUBAR_HEIGHT, 0);
    db.set_image(Some(di));
    db.set_frame(FLAT_BUTTON_FRAME_TYPE);
    db.set_down_frame(DOWN_BUTTON_FRAME_TYPE);
    db.set_tooltip("Delete the Reward");

    logic(&mut db);

    db
}

/// Set a callback and a handler for the delete button
fn logic<T: ButtonExt + WidgetBase + 'static>(db: &mut T) {
    // The callback is disabled by default since there
    // is no reward selection at the start of the program
    db.set_callback(|_| {});
    // Handle the shortcut, lock, and focus / selection events
    db.handle({
        // This button is locked by default. Selecting an item will unlock it
        let mut lock = true;
        move |db, ev| {
            let bits = ev.bits();
            logic::handle_shortcut(db, bits, events::DELETE_BUTTON_SHORTCUT)
                || logic::handle_lock(
                    db,
                    &mut lock,
                    callback,
                    bits,
                    events::LOCK_THE_DELETE_A_REWARD_BUTTON,
                    events::UNLOCK_THE_DELETE_A_REWARD_BUTTON,
                )
                || logic::handle_button(db, ev, 1, lock)
        }
    });
}

/// The callback of the delete button when it's active
fn callback<T: WidgetBase>(_: &mut T) {
    // Send a signal to the Rewards' list to delete the selected reward
    app::handle_main(events::DELETE_A_REWARD).ok();
}
