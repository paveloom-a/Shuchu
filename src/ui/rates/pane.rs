//! The pane itself. It is supposed to be the second pane in the main window.

use fltk::{group::Pack, prelude::*};

use super::{list, menubar};
use crate::ui::constants::FOCUS_PANE_HEIGHT;

/// Initialize the pane
pub fn pane() -> Pack {
    let mut p = Pack::default().with_pos(10, 10 + FOCUS_PANE_HEIGHT + 10);
    p.set_spacing(1);

    // If this pane is a child of the Main Window
    if let Some(ref w) = p.parent() {
        // Set the size, excluding the pane's margins and the height taken by the Focus Pane
        p.set_size(
            w.width() - 20,
            w.height() - 10 - FOCUS_PANE_HEIGHT - 10 - 10,
        );
    }

    // Initialize the widgets
    let _menubar = menubar::new();
    let list = list::new();

    p.end();

    // The list's Scroll is handling the resizing
    p.resizable(list.scroll());

    // This pane is hidden by default. Pressing the Arrow button will bring it up
    p.hide();

    p
}
