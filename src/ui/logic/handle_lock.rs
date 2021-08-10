//! Handle custom Lock / Unlock events for the button.

use fltk::prelude::*;

use super::{mouse_hovering, select_active, select_inactive};

/// Handle Lock / Unlock events for the button
pub fn handle_lock<T: WidgetBase + ButtonExt, F: 'static>(
    b: &mut T,
    lock: &mut bool,
    callback: F,
    bits: i32,
    lock_event: i32,
    unlock_event: i32,
) -> bool
where
    F: Fn(&mut T),
{
    // In case of a lock event
    if bits == lock_event {
        // Lock and remove the callback
        *lock = true;
        b.set_callback(|_| {});
        // Redraw as an inactive button if hovering it
        if mouse_hovering(b) {
            select_inactive(b);
        }
        true
    // In case of an unlock event
    } else if bits == unlock_event {
        // Unlock and reset the callback
        *lock = false;
        b.set_callback(callback);
        // Redraw as an active button if hovering it
        if mouse_hovering(b) {
            select_active(b);
        }
        true
    // Otherwise, do nothing
    } else {
        false
    }
}
