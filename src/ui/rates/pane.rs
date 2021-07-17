use fltk::{group::Pack, prelude::*};

use super::{list, menubar};
use crate::ui::app::CONSTANTS;

pub fn pane() -> Pack {
    let mut pane = Pack::default().with_pos(10, 10 + CONSTANTS.focus_pane_height + 10);
    pane.set_spacing(1);

    if let Some(ref parent) = pane.parent() {
        pane.set_size(
            parent.width() - 20,
            parent.height() - CONSTANTS.focus_pane_height - CONSTANTS.rewards_menubar_height,
        );
    }

    let _menubar = menubar();
    let list = list();

    pane.end();
    pane.resizable(list.scroll());
    pane.hide();
    pane
}
