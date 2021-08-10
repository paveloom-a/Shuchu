//! Handle [`Key::Tab`](fltk::enums::Key::Tab) events for the buttons.

use fltk::prelude::*;

/// Handle [`Key::Tab`](fltk::enums::Key::Tab) events for the buttons
pub fn handle_tab<T: ButtonExt>(b: &mut T) -> bool {
    // The idea here is to block the handling of the `Tab` event (by
    // returning `true`) if neither of the secondary panes is visible.
    // If that's `false`, the pane's `Tab` handling will be used
    b.parent().map_or(false, |p| {
        p.parent().map_or(false, |w| {
            w.child(1).map_or(false, |re_p| {
                w.child(2)
                    .map_or(false, |ra_p| !(re_p.visible() || ra_p.visible()))
            })
        })
    })
}
