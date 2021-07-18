use fltk::{group::Pack, prelude::*};

use super::{list, menubar};
use crate::ui::app::CONSTANTS;

pub fn pane() -> Pack {
    let mut p = Pack::default().with_pos(10, 10 + CONSTANTS.focus_pane_height + 10);
    p.set_spacing(1);

    if let Some(ref w) = p.parent() {
        p.set_size(
            w.width() - 20,
            w.height() - CONSTANTS.focus_pane_height - CONSTANTS.rewards_menubar_height,
        );
    }

    let _menubar = menubar();
    let list = list();

    p.end();
    p.resizable(list.scroll());
    p.hide();
    p
}
