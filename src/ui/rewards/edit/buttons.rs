use fltk::{
    group::{Pack, PackType},
    prelude::*,
};

use super::{cancel, ok};
use crate::ui::app::CONSTANTS;

pub fn buttons() -> Pack {
    let mut bs = Pack::default()
        .with_pos(
            CONSTANTS.rewards_edit_window_width - 10 - 2 * 80 - 10,
            CONSTANTS.rewards_edit_window_height - 20 - 15,
        )
        .with_size(2 * 80, 25);
    bs.set_spacing(10);
    bs.set_type(PackType::Horizontal);

    let _cancel = cancel();
    let _ok = ok();

    bs.end();
    bs
}
