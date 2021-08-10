//! Reward input sets the reward text in a reward.

use fltk::{
    app,
    enums::{Align, Event, FrameType, Key},
    input::Input,
    prelude::*,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::constants::REWARDS_EDIT_WINDOW_WIDTH;

/// Initialize the reward input
pub fn reward(channels: &Channels) -> Input {
    let mut r = Input::new(10, 70, REWARDS_EDIT_WINDOW_WIDTH - 20, 20, "Reward:");
    r.set_frame(FrameType::BorderBox);
    r.set_align(Align::TopLeft);
    r.set_label_size(16);
    r.set_text_size(16);

    logic(&mut r, channels);

    r
}

/// Set a callback and a handler for the reward input
fn logic<T: WidgetBase + InputExt>(i: &mut T, channels: &Channels) {
    // Handle standard and custom events
    i.handle({
        // Receive coins
        let r_coins = channels.rewards_send_coins.r.clone();
        // Receive a reward
        let r_reward = channels.rewards_receive_reward.r.clone();
        // Send an item
        let s_item = channels.rewards_send_item.s.clone();
        // Send signals to the main window
        let s_mw = channels.mw.s.clone();
        move |i, ev| match ev {
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
                i.set_value("");
                true
            }
            // In case of the
            _ => match ev.bits() {
                // 'Add a reward: send reward' event, receive the coins from the coins valuator
                events::ADD_A_REWARD_SEND_REWARD => r_coins.try_recv().map_or(false, |coins| {
                    // Create an Item String and send it to the Rewards' list
                    s_item.try_send(format!("({}) {}", coins, i.value())).ok();
                    // Pass a signal to the Reward's list to receive the item
                    s_mw.try_send(events::ADD_A_REWARD_RECEIVE).ok();
                    // Reset the input
                    i.set_value("");
                    // Hide the window
                    if let Some(ref mut w) = i.parent() {
                        w.hide();
                    }
                    true
                }),
                // 'Edit a reward: send reward' event, receive the coins from the coins valuator
                events::EDIT_THE_REWARD_SEND_REWARD => r_coins.try_recv().map_or(false, |coins| {
                    // Create an Item String and send it to the Rewards' list
                    s_item.try_send(format!("({}) {}", coins, i.value())).ok();
                    // Pass a signal to the Reward's list to receive the item
                    s_mw.try_send(events::EDIT_THE_REWARD_RECEIVE).ok();
                    // Reset the input
                    i.set_value("");
                    // Hide the window
                    if let Some(ref mut w) = i.parent() {
                        w.hide();
                    }
                    true
                }),
                // 'Edit a reward: receive reward' event
                events::EDIT_THE_REWARD_RECEIVE_REWARD => {
                    // Receive the reward text
                    r_reward.try_recv().map_or(false, |ref reward| {
                        // Set it as a new value of the input
                        i.set_value(reward);
                        true
                    })
                }
                _ => false,
            },
        }
    });
}
