//! Delete Button deletes the selected item in the [Rates](super::super)' List.

use fltk::{button::Button, image::SvgImage, prelude::*};

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
    db.set_tooltip("Delete the Rate");

    logic(&mut db);

    db
}

/// Set a callback and a handler for the delete button
fn logic<T: ButtonExt + WidgetBase + 'static>(db: &mut T) {
    db.set_callback(|_| {
        println!("Delete Pressed!");
    });
    // Handle the shortcut and focus / selection events
    db.handle(|db, ev| {
        logic::handle_shortcut(db, ev.bits(), events::DELETE_BUTTON_SHORTCUT)
            || logic::handle_button(db, ev, 1, false)
    });
}
