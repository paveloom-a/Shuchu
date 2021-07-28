use fltk::{group::Pack, prelude::*};

use super::{list, menubar};
use crate::ui::app::CONSTANTS;

pub fn pane() -> Pack {
    let mut p = Pack::default().with_pos(10, 10 + CONSTANTS.focus_pane_height + 10);
    p.set_spacing(1);

    // If this pane is a child of the Main Window
    if let Some(ref w) = p.parent() {
        // Set the size, excluding the pane's margins and the height taken by the Focus Pane
        p.set_size(
            w.width() - 20,
            w.height() - 10 - CONSTANTS.focus_pane_height - 10 - 10,
        );
    }

    let _menubar = menubar();
    let list = list();

    p.end();
    p.resizable(list.scroll());
    p.hide();
    p
}
