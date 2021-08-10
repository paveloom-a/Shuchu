//! Rates' List contains the conversion rates.

use fltk::prelude::*;

use crate::ui::constants::REWARDS_MENUBAR_HEIGHT;
use crate::ui::widgets::list::List;

/// Initialize the list
pub fn new() -> List {
    let mut l = List::default();

    // If this list is a child of the Rates pane
    if let Some(ref p) = l.parent() {
        // Set the list's height to all of the available parent's height
        l.set_size(0, p.h() - REWARDS_MENUBAR_HEIGHT);
    }

    // Add examples (temporary)
    l.add("5/m");
    l.add("0.1/s");

    // Select the first one. This list is supposed to always have a selected item
    l.select(1);

    l
}
