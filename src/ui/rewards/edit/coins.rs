use fltk::{
    app,
    enums::{Align, CallbackTrigger, FrameType},
    prelude::*,
    valuator::ValueInput,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::app::CONSTANTS;

pub fn coins(channels: &Channels) -> ValueInput {
    let mut coins = ValueInput::new(
        10,
        25,
        CONSTANTS.rewards_edit_window_width - 20,
        20,
        "Coins:",
    );
    coins.set_frame(FrameType::BorderBox);
    coins.set_align(Align::TopLeft);
    coins.set_label_size(16);
    coins.set_text_size(16);
    coins.set_step(0.0, 5);
    coins.set_precision(2);
    coins.set_bounds(0.0, 999_999_999.0);
    coins.set_range(0.0, 999_999_999.0);
    coins.set_soft(false);
    coins.set_value(0.1);
    coins.set_value(0.0);

    logic(&mut coins, channels);

    coins
}

fn logic<T: WidgetBase + ValuatorExt>(v: &mut T, channels: &Channels) {
    v.set_trigger(CallbackTrigger::Changed);
    v.set_callback(|v| {
        v.set_value(v.clamp(v.value()));
    });
    v.handle({
        let s = channels.rewards_coins.s.clone();
        move |v, ev| match ev.bits() {
            events::ADD_A_REWARD_SEND_COINS => {
                s.try_send(v.value()).ok();
                app::handle_main(events::ADD_A_REWARD_SEND_REWARD).ok();
                v.set_value(0.0);
                true
            }
            events::ADD_A_REWARD_RESET_COINS => {
                v.set_value(0.0);
                true
            }
            _ => false,
        }
    })
}
