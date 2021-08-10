//! Coins valuator sets the number of coins in a reward.
//!
//! Dragging will change the value in the available range.

use fltk::{
    app,
    enums::{Align, CallbackTrigger, Event, FrameType, Key},
    prelude::*,
    valuator::ValueInput,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::constants::REWARDS_EDIT_WINDOW_WIDTH;

/// Initialize the coins valuator
pub fn coins(channels: &Channels) -> ValueInput {
    let mut c = ValueInput::new(10, 25, REWARDS_EDIT_WINDOW_WIDTH - 20, 20, "Coins:");
    c.set_frame(FrameType::BorderBox);
    c.set_align(Align::TopLeft);
    c.set_label_size(16);
    c.set_text_size(16);
    c.set_step(0.0, 5);
    c.set_precision(2);
    c.set_bounds(0.0, 999_999_999.0);
    c.set_range(0.0, 999_999_999.0);
    c.set_soft(false);

    // This trick makes the default value display as `0.00` instead of `0`
    c.set_value(0.1);
    c.set_value(0.0);

    logic(&mut c, channels);

    c
}

/// Set a trigger, a callback and a handler for the coins valuator
fn logic<T: WidgetBase + ValuatorExt>(v: &mut T, channels: &Channels) {
    // The value is clamped as soon as it changes
    v.set_trigger(CallbackTrigger::Changed);
    // The callback is to clamp the valuator
    v.set_callback(|v| {
        v.set_value(v.clamp(v.value()));
    });
    // Handle standard and custom events
    v.handle({
        // Send coins
        let s_coins = channels.rewards_send_coins.s.clone();
        // Receive coins
        let r_coins = channels.rewards_receive_coins.r.clone();
        move |v, ev| match ev {
            // Holding `Enter` down is ignored
            Event::KeyDown => app::event_key() == Key::Enter,
            // In case of a released button
            Event::KeyUp => {
                // If it's `Enter`
                if app::event_key() == Key::Enter {
                    // Do the `OK` button's callback
                    app::handle_main(events::OK_BUTTON_DO_CALLBACK).ok();
                    true
                } else {
                    false
                }
            }
            // Hiding the widget will reset its value
            Event::Hide => {
                v.set_value(0.0);
                true
            }
            // In case of the
            _ => match ev.bits() {
                // 'Add a reward: send coins' event
                events::ADD_A_REWARD_SEND_COINS => {
                    // Get the coins from the valuator
                    s_coins.try_send(v.value()).ok();
                    // Pass a signal to the Reward widget in the same window
                    app::handle_main(events::ADD_A_REWARD_SEND_REWARD).ok();
                    // Reset the valuator (the window will become hidden after this chain of events)
                    v.set_value(0.0);
                    true
                }
                // 'Edit a reward: send coins' event
                events::EDIT_THE_REWARD_SEND_COINS => {
                    // Get the coins from the valuator
                    s_coins.try_send(v.value()).ok();
                    // Pass a signal to the Reward widget in the same window
                    app::handle_main(events::EDIT_THE_REWARD_SEND_REWARD).ok();
                    // Reset the valuator (the window will become hidden after this chain of events)
                    v.set_value(0.0);
                    true
                }
                // 'Edit a reward: receive coins` event, receive the coins
                events::EDIT_THE_REWARD_RECEIVE_COINS => {
                    r_coins.try_recv().map_or(false, |coins| {
                        // Set them as a new value of the valuator
                        v.set_value(coins);
                        true
                    })
                }
                _ => false,
            },
        }
    });
}
