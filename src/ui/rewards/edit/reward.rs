use fltk::{
    enums::{Align, FrameType},
    input::Input,
    prelude::*,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::app::CONSTANTS;

pub fn reward(channels: &Channels) -> Input {
    let mut reward = Input::new(
        10,
        70,
        CONSTANTS.rewards_edit_window_width - 20,
        20,
        "Reward:",
    );
    reward.set_frame(FrameType::BorderBox);
    reward.set_align(Align::TopLeft);
    reward.set_label_size(16);
    reward.set_text_size(16);

    logic(&mut reward, channels);

    reward
}

fn logic<T: WidgetBase + InputExt>(i: &mut T, channels: &Channels) {
    i.handle({
        let r_coins = channels.rewards_coins.r.clone();
        let s_reward = channels.rewards_reward.s.clone();
        let s_mw = channels.mw.s.clone();
        move |i, ev| match ev.bits() {
            events::ADD_A_REWARD_SEND_REWARD => r_coins.try_recv().map_or(false, |coins| {
                s_reward.try_send(format!("({}) {}", coins, i.value())).ok();
                s_mw.try_send(events::ADD_A_REWARD_RECEIVE).ok();
                i.set_value("");
                true
            }),
            events::ADD_A_REWARD_RESET_REWARD => {
                i.set_value("");
                true
            }
            _ => false,
        }
    });
}
