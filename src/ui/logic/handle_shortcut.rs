//! Handle custom Shortcut events for the button.

use fltk::prelude::*;

/// Handle custom shortcut events for the button
pub fn handle_shortcut<T: ButtonExt>(b: &mut T, bits: i32, shortcut_event: i32) -> bool {
    if bits == shortcut_event {
        b.do_callback();
        true
    } else {
        false
    }
}
