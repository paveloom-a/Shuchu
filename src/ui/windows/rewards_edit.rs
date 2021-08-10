//! Rewards Edit window provides a way to add / edit items in the [Rewards](super::super::rewards)
//! list.
//!
//! This secondary window is called by the 'Add a Reward' and 'Edit the Reward' buttons in the
//! Rewards' Menubar.

use fltk::app;
use fltk::{prelude::*, window::Window};

use super::icon;
use crate::channels::Channels;
use crate::events;
use crate::ui::constants::{REWARDS_EDIT_WINDOW_HEIGHT, REWARDS_EDIT_WINDOW_WIDTH};
use crate::ui::logic;
use crate::ui::rewards::edit;

/// Initialize the Rewards Edit Window
pub fn rewards_edit(channels: &Channels) -> Window {
    let mut w = Window::new(
        100,
        100,
        REWARDS_EDIT_WINDOW_WIDTH,
        REWARDS_EDIT_WINDOW_HEIGHT,
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

    logic(&mut w, channels);

    w
}

/// Set a handle for the window
fn logic<T: WidgetBase>(w: &mut T, channels: &Channels) {
    w.handle({
        let r_item = channels.rewards_receive_item.r.clone();
        let s_coins = channels.rewards_receive_coins.s.clone();
        let s_reward = channels.rewards_receive_reward.s.clone();
        // In case of the
        move |w, ev| match ev.bits() {
            // 'Add a reward` event
            events::ADD_A_REWARD_OPEN => {
                w.set_label("Add a Reward");
                w.show();
                // Set the OK button's callback to 'Add a reward' (this is done after the window
                // is shown, so that the OK button is able to receive it)
                app::handle_main(events::OK_BUTTON_SET_TO_ADD).ok();
                true
            }
            // 'Edit the reward' event
            events::EDIT_THE_REWARD_OPEN => {
                w.set_label("Edit the Reward");
                w.show();
                // Set the OK button's callback to 'Edit a reward' (this is done after the window
                // is shown, so that the OK button is able to receive it)
                app::handle_main(events::OK_BUTTON_SET_TO_EDIT).ok();
                // Get the item
                if let Ok(item) = r_item.try_recv() {
                    // Get the price and the text
                    if let Some(price) = logic::get_price(&item) {
                        if let Some(text) = logic::get_text(&item) {
                            // Send the price to the Reward Edit's Coins widget
                            s_coins.try_send(price).ok();
                            app::handle_main(events::EDIT_THE_REWARD_RECEIVE_COINS).ok();

                            // Send the text to the Reward Edit's Reward widget
                            s_reward.try_send(text).ok();
                            app::handle_main(events::EDIT_THE_REWARD_RECEIVE_REWARD).ok();
                        }
                    }
                }
                true
            }
            _ => false,
        }
    });
}
