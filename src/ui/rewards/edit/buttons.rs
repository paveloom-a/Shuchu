//! The buttons pack contains the [`Ok`](mod@super::ok) and [`Cancel`](mod@super::cancel) buttons.

use fltk::{
    group::{Pack, PackType},
    prelude::*,
};

use super::{cancel, ok};
use crate::ui::constants::{REWARDS_EDIT_WINDOW_HEIGHT, REWARDS_EDIT_WINDOW_WIDTH};

/// Initialize the buttons pack
pub fn buttons() -> Pack {
    // 80px is the width of the buttons, 25px — the height, 10px — the margins of the pack
    let mut bs = Pack::default()
        .with_pos(
            REWARDS_EDIT_WINDOW_WIDTH - 10 - 2 * 80 - 10,
            REWARDS_EDIT_WINDOW_HEIGHT - 25 - 10,
        )
        .with_size(2 * 80 + 10, 25);
    bs.set_spacing(10);
    bs.set_type(PackType::Horizontal);

    // Initialize the buttons
    let _cancel = cancel();
    let _ok = ok();

    bs.end();
    bs
}
