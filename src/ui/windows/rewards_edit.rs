use fltk::app;
use fltk::{prelude::*, window::Window};

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

    logic(&mut w, channels);

    w
}

fn logic<T: WidgetBase>(w: &mut T, channels: &Channels) {
    w.handle({
        let r_item = channels.rewards_receive_item.r.clone();
        let s_coins = channels.rewards_receive_coins.s.clone();
        let s_reward = channels.rewards_receive_reward.s.clone();
        move |w, ev| match ev.bits() {
            events::ADD_A_REWARD_OPEN => {
                w.show();
                true
            }
            events::EDIT_A_REWARD_OPEN => {
                w.show();
                if let Ok(item) = r_item.try_recv() {
                    if let Some(rb_i) = item.find(')') {
                        if let Ok(coins) = item[1..rb_i].parse::<f64>() {
                            s_coins.try_send(coins).ok();
                            app::handle_main(events::EDIT_A_REWARD_RECEIVE_COINS).ok();
                            s_reward.try_send(item[(rb_i + 2)..].to_string()).ok();
                            app::handle_main(events::EDIT_A_REWARD_RECEIVE_REWARD).ok();
                        }
                    }
                }
                true
            }
            _ => false,
        }
    })
}
