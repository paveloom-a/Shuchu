use fltk::prelude::*;

use crate::ui::app::CONSTANTS;
use crate::ui::widgets::list::List;

pub fn list() -> List {
    let mut l = List::default();

    // If this list is a child of the Rates pane
    if let Some(ref p) = l.parent() {
        // Set the list's height to everything except the Menubar's height
        l.set_size(0, p.h() - CONSTANTS.rewards_menubar_height);
    }

    l.add("5/m");
    l.add("0.1/s");
    l.select(1);

    l
}
