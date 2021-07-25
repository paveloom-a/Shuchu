use fltk::{
    app,
    enums::{Align, CallbackTrigger, Event, FrameType, Key},
    prelude::*,
    valuator::ValueInput,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::app::CONSTANTS;

pub fn coins(channels: &Channels) -> ValueInput {
    let mut c = ValueInput::new(
        10,
        25,
        CONSTANTS.rewards_edit_window_width - 20,
        20,
        "Coins:",
    );
    c.set_frame(FrameType::BorderBox);
    c.set_align(Align::TopLeft);
    c.set_label_size(16);
    c.set_text_size(16);
    c.set_step(0.0, 5);
    c.set_precision(2);
    c.set_bounds(0.0, 999_999_999.0);
    c.set_range(0.0, 999_999_999.0);
    c.set_soft(false);
    c.set_value(0.1);
    c.set_value(0.0);

    logic(&mut c, channels);

    c
}

fn logic<T: WidgetBase + ValuatorExt>(v: &mut T, channels: &Channels) {
    v.set_trigger(CallbackTrigger::Changed);
    v.set_callback(|v| {
        v.set_value(v.clamp(v.value()));
    });
    v.handle({
        let s_coins = channels.rewards_send_coins.s.clone();
        let r_coins = channels.rewards_receive_coins.r.clone();
        move |v, ev| match ev {
            Event::KeyDown => app::event_key() == Key::Enter,
            Event::KeyUp => {
                if app::event_key() == Key::Enter {
                    app::handle_main(events::OK_BUTTON_DO_CALLBACK).ok();
                    true
                } else {
                    false
                }
            }
            Event::Hide => {
                v.set_value(0.0);
                true
            }
            _ => match ev.bits() {
                events::ADD_A_REWARD_SEND_COINS => {
                    s_coins.try_send(v.value()).ok();
                    app::handle_main(events::ADD_A_REWARD_SEND_REWARD).ok();
                    v.set_value(0.0);
                    true
                }
                events::EDIT_A_REWARD_SEND_COINS => {
                    s_coins.try_send(v.value()).ok();
                    app::handle_main(events::EDIT_A_REWARD_SEND_REWARD).ok();
                    v.set_value(0.0);
                    true
                }
                events::EDIT_A_REWARD_RECEIVE_COINS => r_coins.try_recv().map_or(false, |coins| {
                    v.set_value(coins);
                    true
                }),
                _ => false,
            },
        }
    })
}
