//! Handle [`Key::Right`](fltk::enums::Key::Right) events for the button.

use fltk::prelude::*;

/// Handle Right events for the button
pub fn handle_right<T: ButtonExt>(b: &mut T, idx: i32) -> bool {
    if let Some(ref p) = b.parent() {
        let max = p.children() - 1;
        let next_idx = get_next_idx(idx, max);
        if let Some(ref mut cw) = p.child(next_idx) {
            // This one-time check is needed to jump over menu dividers
            if cw.has_visible_focus() {
                cw.take_focus().ok();
            } else if let Some(ref mut ncw) = p.child(get_next_idx(next_idx, max)) {
                if ncw.has_visible_focus() {
                    ncw.take_focus().ok();
                }
            }
        }
    }
    true
}

/// Get the next index
fn get_next_idx(idx: i32, max: i32) -> i32 {
    if idx == max {
        0
    } else {
        idx + 1
    }
}
