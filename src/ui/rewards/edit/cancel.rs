//! Cancel button cancels the adding / editing process and hides the window.
//!
//! The inputs are then reset by the widgets' `Hide` event handlers.

use fltk::{button::Button, prelude::*};

use crate::ui::constants::{BOXED_BUTTON_FRAME_TYPE, DOWN_BUTTON_FRAME_TYPE};
use crate::ui::logic;

/// Initialize the cancel button
pub fn cancel() -> Button {
    let mut c = Button::default().with_size(80, 0).with_label("Cancel");
    c.set_frame(BOXED_BUTTON_FRAME_TYPE);
    c.set_down_frame(DOWN_BUTTON_FRAME_TYPE);

    logic(&mut c);

    c
}

/// Set a callback and a handler for the cancel button
fn logic<T: WidgetBase + ButtonExt>(b: &mut T) {
    // Pushing the button will hide the window
    b.set_callback(|b| {
        if let Some(ref p) = b.parent() {
            if let Some(ref mut w) = p.parent() {
                w.hide();
            }
        }
    });
    // Handle focus / selection events
    b.handle(|c, ev| logic::handle_selection(c, ev, BOXED_BUTTON_FRAME_TYPE, false));
}
