use fltk::{
    enums::{Align, Event, FrameType},
    input::Input,
    prelude::*,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::app::CONSTANTS;

pub fn reward(channels: &Channels) -> Input {
    let mut r = Input::new(
        10,
        70,
        CONSTANTS.rewards_edit_window_width - 20,
        20,
        "Reward:",
    );
    r.set_frame(FrameType::BorderBox);
    r.set_align(Align::TopLeft);
    r.set_label_size(16);
    r.set_text_size(16);

    logic(&mut r, channels);

    r
}

fn logic<T: WidgetBase + InputExt>(i: &mut T, channels: &Channels) {
    i.handle({
        let r_coins = channels.rewards_send_coins.r.clone();
        let r_reward = channels.rewards_receive_reward.r.clone();
        let s_item = channels.rewards_send_item.s.clone();
        let s_mw = channels.mw.s.clone();
        move |i, ev| match ev {
            Event::Hide => {
                i.set_value("");
                true
            }
            _ => match ev.bits() {
                events::ADD_A_REWARD_SEND_REWARD => r_coins.try_recv().map_or(false, |coins| {
                    s_item.try_send(format!("({}) {}", coins, i.value())).ok();
                    s_mw.try_send(events::ADD_A_REWARD_RECEIVE).ok();
                    i.set_value("");
                    if let Some(ref mut w) = i.parent() {
                        w.hide();
                    }
                    true
                }),
                events::EDIT_A_REWARD_SEND_REWARD => r_coins.try_recv().map_or(false, |coins| {
                    s_item.try_send(format!("({}) {}", coins, i.value())).ok();
                    s_mw.try_send(events::EDIT_A_REWARD_RECEIVE).ok();
                    i.set_value("");
                    if let Some(ref mut w) = i.parent() {
                        w.hide();
                    }
                    true
                }),
                events::EDIT_A_REWARD_RECEIVE_REWARD => {
                    r_reward.try_recv().map_or(false, |ref reward| {
                        i.set_value(reward);
                        true
                    })
                }
                _ => false,
            },
        }
    });
}
