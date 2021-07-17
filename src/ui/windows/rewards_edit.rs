use fltk::{enums::Event, prelude::*, window::Window};

use super::icon;
use crate::channels::Channels;
use crate::events;
use crate::ui::app::CONSTANTS;
use crate::ui::rewards::edit;

/// Create the Rewards Edit Window
pub fn rewards_edit(channels: &Channels) -> Window {
    let mut window = Window::new(
        100,
        100,
        CONSTANTS.rewards_edit_window_width,
        CONSTANTS.rewards_edit_window_height,
        "Add a Reward",
    )
    .center_screen();
    window.set_icon(Some(icon()));
    window.make_modal(true);

    // 1. Coins
    let _coins = edit::coins(channels);

    // 2. Reward
    let _reward = edit::reward(channels);

    // 3. Buttons
    let _buttons = edit::buttons();

    window.end();

    window.handle(handle);

    window
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
