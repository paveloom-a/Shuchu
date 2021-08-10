//! Add Button opens the Rates Edit window, set to add a new item.

use fltk::{button::Button, image::SvgImage, prelude::*};

use crate::events;
use crate::ui::constants::{
    DOWN_BUTTON_FRAME_TYPE, FLAT_BUTTON_FRAME_TYPE, REWARDS_MENUBAR_HEIGHT,
};
use crate::ui::logic;

/// Initialize the add button
pub fn add_button() -> Button {
    let mut ai = SvgImage::from_data(include_str!("../../../../assets/menu/plus.svg")).unwrap();
    ai.scale(24, 24, true, true);

    let mut ab = Button::default().with_size(REWARDS_MENUBAR_HEIGHT, 0);
    ab.set_image(Some(ai));
    ab.set_frame(FLAT_BUTTON_FRAME_TYPE);
    ab.set_down_frame(DOWN_BUTTON_FRAME_TYPE);
    ab.set_tooltip("Add a Rate");

    logic(&mut ab);

    ab
}

/// Set a callback and a handler for the add button
fn logic<T: ButtonExt + WidgetBase + 'static>(ab: &mut T) {
    ab.set_callback(|_| {
        println!("Add Pressed!");
    });
    // Handle the shortcut and focus / selection events
    ab.handle(|ab, ev| {
        logic::handle_shortcut(ab, ev.bits(), events::ADD_BUTTON_SHORTCUT)
            || logic::handle_button(ab, ev, 0, false)
    });
}
