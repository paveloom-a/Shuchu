use fltk::{enums::Event, prelude::*, window::Window};

use super::icon;
use crate::channels::Channels;
use crate::events;
use crate::ui::app::CONSTANTS;
use crate::ui::rewards::edit;

/// Create the Rewards Edit Window
pub fn rewards_edit(channels: &Channels) -> Window {
    let mut w = Window::new(
        100,
        100,
        CONSTANTS.rewards_edit_window_width,
        CONSTANTS.rewards_edit_window_height,
        "Add a Reward",
    )
    .center_screen();
    w.set_icon(Some(icon()));
    w.make_modal(true);

    // 1. Coins
    let _c = edit::coins(channels);

    // 2. Reward
    let _r = edit::reward(channels);

    // 3. Buttons
    let _bs = edit::buttons();

    w.end();

    w.handle(handle);

    w
}

fn handle<T: WidgetBase>(w: &mut T, ev: Event) -> bool {
    match ev.bits() {
        events::ADD_A_REWARD_OPEN => {
            w.show();
            true
        }
        _ => false,
    }
}
