//! Handle [`Key::Left`](fltk::enums::Key::Left) events for the button.

use fltk::prelude::*;

/// Handle Left events for the button
pub fn handle_left<T: ButtonExt>(b: &mut T, idx: i32) -> bool {
    if let Some(ref p) = b.parent() {
        let max = p.children() - 1;
        let prev_idx = get_prev_idx(idx, max);
        if let Some(ref mut cw) = p.child(prev_idx) {
            // This one-time check is needed to jump over menu dividers
            if cw.has_visible_focus() {
                cw.take_focus().ok();
            } else if let Some(ref mut pcw) = p.child(get_prev_idx(prev_idx, max)) {
                if pcw.has_visible_focus() {
                    pcw.take_focus().ok();
                }
            }
        }
    }
    true
}

/// Get the previous index
fn get_prev_idx(idx: i32, max: i32) -> i32 {
    if idx == 0 {
        max
    } else {
        idx - 1
    }
}
