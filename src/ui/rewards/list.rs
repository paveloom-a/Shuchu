//! Reward's List keeps the rewards and changes them if requested.

use crossbeam_channel::Sender;
use fltk::{app, enums::Key, group::Scroll, prelude::*};

use crate::channels::Channels;
use crate::events;
use crate::ui::constants::REWARDS_MENUBAR_HEIGHT;
use crate::ui::logic;
use crate::ui::widgets::list::{Holder, Items, List, Selected};

/// Initialize the list
pub fn new(channels: &Channels) -> List {
    let mut l = List::default();

    // If this list is a child of the Rewards pane
    if let Some(ref p) = l.parent() {
        // Set the list's height to all of the available parent's height
        l.set_size(0, p.h() - REWARDS_MENUBAR_HEIGHT);
    }

    // Add examples (temporary)
    l.add("(15) A reward.");
    l.add("(10) Another reward.");
    l.add("(5) One more reward.");

    // Apply custom logic
    l.handle(
        handle_selection_closure(channels),
        handle_selection_closure(channels),
        handle_custom_events(channels),
    );

    l
}

/// Handle custom selection events in the returned closure
fn handle_selection_closure(channels: &Channels) -> impl Fn(&mut Scroll, &Selected, &Items) {
    // Send the price of an item
    let s_price = channels.rewards_receive_coins.s.clone();
    move |_, selected, items| handle_selection(selected, items, &s_price)
}

/// Handle custom selection events
fn handle_selection(selected: &Selected, items: &Items, s_price: &Sender<f64>) {
    // If there is a selected item, and the event is a click or an Up / Down button
    if selected.get() > 0
        && (app::event_is_click() || app::event_key() == Key::Down || app::event_key() == Key::Up)
    {
        // Unlock the buttons
        handle_selection_unlock_buttons();
        // Check if the selected reward is affordable
        handle_selection_check_affordability(selected, items, s_price);
    }
}

/// Unlock the Delete / Edit buttons
fn handle_selection_unlock_buttons() {
    app::handle_main(events::UNLOCK_THE_DELETE_A_REWARD_BUTTON).ok();
    app::handle_main(events::UNLOCK_THE_EDIT_A_REWARD_BUTTON).ok();
}

/// Lock the Delete / Edit / Spend buttons
fn handle_selection_lock_buttons() {
    app::handle_main(events::LOCK_THE_DELETE_A_REWARD_BUTTON).ok();
    app::handle_main(events::LOCK_THE_EDIT_A_REWARD_BUTTON).ok();
    app::handle_main(events::LOCK_THE_SPEND_BUTTON).ok();
}

/// Check if the selected item is affordable
fn handle_selection_check_affordability(selected: &Selected, items: &Items, s_price: &Sender<f64>) {
    let index = selected.index();
    // Get the price of the item
    if let Some(price) = logic::get_price(&*items.index(index).string()) {
        // Send it to the Coins widget for an affordability check
        s_price.try_send(price).ok();
        app::handle_main(events::CHECK_AFFORDABILITY_RECEIVE_PRICE).ok();
    }
}

/// Handle custom events in Rewards
fn handle_custom_events(
    channels: &Channels,
) -> impl Fn(&mut Scroll, &Holder, &Selected, &Items, i32) -> bool {
    // Send signals to the Rewards Edit window
    let s_re = channels.rewards_edit.s.clone();
    // Send the price of an item
    let s_price = channels.rewards_receive_coins.s.clone();
    // Send an item
    let s_item = channels.rewards_receive_item.s.clone();
    // Receive an item
    let r_item = channels.rewards_send_item.r.clone();

    // In case of the
    move |scroll, holder, selected, items, bits| match bits {
        // 'Add a reward' event
        events::ADD_A_REWARD_RECEIVE => r_item.try_recv().map_or(false, |string| {
            // Add a new reward
            holder.add(string, scroll, items);
            true
        }),
        // 'Delete a reward' event
        events::DELETE_A_REWARD => {
            // If there is a selected item
            if selected.get() > 0 {
                // Get its index
                let index = selected.index();
                // If the selected item is the last one in the list
                if selected.get() == items.len() {
                    // Decrement the selection
                    selected.decrement();
                }
                // If that's the only item in the list
                if items.len() == 1 {
                    // Lock the Edit / Delete / Spend buttons
                    handle_selection_lock_buttons();
                // Otherwise,
                } else {
                    // Handle the custom selection
                    handle_selection(selected, items, &s_price);
                }
                // Remove the item from the list
                holder.remove(index, scroll, items);
                // Update the selection
                items.select(selected.get());
            }
            true
        }
        // 'Edit a reward: receive the item' event, receive the item
        events::EDIT_THE_REWARD_RECEIVE => r_item.try_recv().map_or(false, |string| {
            // Update the selected item
            items.index_mut(selected.index()).set(string);
            // Check the affordability of the selected item
            handle_selection_check_affordability(selected, items, &s_price);
            // Redraw the holder
            holder.redraw();
            true
        }),
        // 'Edit a reward: send the item' event
        events::EDIT_THE_REWARD_SEND_ITEM => {
            // If there is a selected item
            if selected.get() > 0 {
                // Get a copy of its Item String
                let string = items.index(selected.index()).clone();
                // Send it to the Rewards Edit window
                s_item.try_send(string).ok();
                s_re.try_send(events::EDIT_THE_REWARD_OPEN).ok();
                // Set the Rewards Edit window to the Edit mode
                s_re.try_send(events::OK_BUTTON_SET_TO_EDIT).ok();
            }
            true
        }
        // 'Spend coins: send price' event
        events::SPEND_COINS_SEND_PRICE => {
            // If there is a selected item
            if selected.get() > 0 {
                // Get the price of the item (or set it to negative if couldn't parse it)
                let price = if let Some(price) =
                    logic::get_price(&*items.index(selected.index()).string())
                {
                    price
                } else {
                    -1.0
                };
                // Using this check instead of `let Some()` because
                // the list gets mutated later in the chain of events
                if price >= 0.0 {
                    // Send it to the Coins widget
                    s_price.try_send(price).ok();
                    app::handle_main(events::SPEND_COINS_RECEIVE_PRICE).ok();
                }
            }
            true
        }
        _ => false,
    }
}
