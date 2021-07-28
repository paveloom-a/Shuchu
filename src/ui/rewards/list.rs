use crossbeam_channel::Sender;
use fltk::{
    app,
    group::{Pack, Scroll},
    prelude::*,
};

use crate::channels::Channels;
use crate::events;
use crate::ui::app::CONSTANTS;
use crate::ui::widgets::list::{Holder, Items, List, Selected};

pub fn new(channels: &Channels) -> List {
    let mut l = List::default();

    // If this list is a child of the Rewards pane
    if let Some(ref p) = l.parent() {
        // Set the list's height to everything except the Menubar's height
        l.set_size(0, p.h() - CONSTANTS.rewards_menubar_height);
    }

    l.add("(15) A reward.");
    l.add("(10) Another reward.");
    l.add("(5) One more reward.");

    logic(&mut l, channels);

    l
}

fn logic(l: &mut List, channels: &Channels) {
    l.handle(
        handle_selection(channels),
        handle_selection(channels),
        handle_custom_events(channels),
    );
}

fn handle_selection(channels: &Channels) -> impl Fn(&mut Scroll, &Selected, &Items) {
    let s_price = channels.rewards_receive_coins.s.clone();
    move |_, selected, items| handle_selection_all(selected, items, &s_price)
}

fn handle_selection_all(selected: &Selected, items: &Items, s_price: &Sender<f64>) {
    if selected.get() > 0 && items.len() > 0 {
        handle_selection_unlock_buttons();
        handle_selection_check_affordability(selected, items, s_price);
    }
}

fn handle_selection_unlock_buttons() {
    app::handle_main(events::UNLOCK_THE_DELETE_A_REWARD_BUTTON).ok();
    app::handle_main(events::UNLOCK_THE_EDIT_A_REWARD_BUTTON).ok();
}

fn handle_selection_lock_buttons() {
    app::handle_main(events::LOCK_THE_DELETE_A_REWARD_BUTTON).ok();
    app::handle_main(events::LOCK_THE_EDIT_A_REWARD_BUTTON).ok();
    app::handle_main(events::LOCK_THE_SPEND_BUTTON).ok();
}

fn handle_selection_check_affordability(selected: &Selected, items: &Items, s_price: &Sender<f64>) {
    let index = selected.index();
    if let Some(rb_i) = items.index(index).find(')') {
        if let Ok(price) = items.index(index).index(1..rb_i).parse::<f64>() {
            s_price.try_send(price).ok();
            app::handle_main(events::CHECK_AFFORDABILITY_RECEIVE_PRICE).ok();
        }
    }
}

fn handle_custom_events(
    channels: &Channels,
) -> impl Fn(&mut Pack, &mut Selected, &mut Items, i32) -> bool {
    let s_re = channels.rewards_edit.s.clone();
    let s_price = channels.rewards_receive_coins.s.clone();
    let s_item = channels.rewards_receive_item.s.clone();
    let r_item = channels.rewards_send_item.r.clone();
    move |h, selected, items, bits| match bits {
        events::ADD_A_REWARD_RECEIVE => r_item.try_recv().map_or(false, |string| {
            Holder::add_to(h, string, items);
            true
        }),
        events::DELETE_A_REWARD => {
            if selected.get() > 0 {
                let index = selected.index();
                if selected.get() == items.len() {
                    selected.decrement();
                }
                if items.len() == 1 {
                    handle_selection_lock_buttons();
                } else {
                    handle_selection_all(selected, items, &s_price);
                }
                Holder::remove(h, index, items);
                items.select(selected.get());
            }
            true
        }
        events::EDIT_A_REWARD_RECEIVE => r_item.try_recv().map_or(false, |string| {
            items.index_mut(selected.index()).set(string);
            handle_selection_check_affordability(selected, items, &s_price);
            h.redraw();
            true
        }),
        events::EDIT_A_REWARD_SEND_ITEM => {
            if selected.get() > 0 {
                let string = items.index(selected.index()).clone();
                s_item.try_send(string).ok();
                s_re.try_send(events::EDIT_A_REWARD_OPEN).ok();
                s_re.try_send(events::OK_BUTTON_SET_TO_EDIT).ok();
            }
            true
        }
        events::SPEND_COINS_SEND_PRICE => {
            if selected.get() > 0 {
                let index = selected.index();
                let rb_i = items.index(index).find(')').unwrap_or_default();
                let price = items
                    .index(index)
                    .index(1..rb_i)
                    .parse::<f64>()
                    .unwrap_or(-1.0);
                if price >= 0.0 {
                    s_price.try_send(price).ok();
                    app::handle_main(events::SPEND_COINS_RECEIVE_PRICE).ok();
                }
            }
            true
        }
        _ => false,
    }
}
