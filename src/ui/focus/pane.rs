use fltk::{enums::FrameType, group::Group, prelude::*};

use super::{arrow, coins, timer};
use crate::ui::app::CONSTANTS;

pub fn pane() -> Group {
    let mut pane = Group::default().with_pos(10, 10);
    pane.set_frame(FrameType::BorderBox);

    if let Some(parent) = pane.parent() {
        pane.set_size(parent.width() - 20, CONSTANTS.focus_pane_height);
    }

    // The order of the widgets is important

    let _timer = timer();
    let _coins = coins();
    let _arrow = arrow();

    pane.end();
    pane
}
