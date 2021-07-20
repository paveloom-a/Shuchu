use crossbeam_channel::Sender;
use fltk::{app, group::Scroll, prelude::*};

use crate::channels::Channels;
use crate::events;
use crate::ui::widgets::{Items, List, ScrollExt, Selected};

pub fn new(channels: &Channels) -> List {
    let mut l = List::default();

    if let Some(ref p) = l.parent() {
        l.set_size(0, p.h() - 20);
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
    move |_, selected, items| {
        handle_selection_closure(selected, items, &s_price);
    }
}

fn handle_selection_closure(selected: &Selected, items: &Items, s_price: &Sender<f64>) {
    if *selected.borrow() > 0 && items.borrow().len() > 0 {
        app::handle_main(events::UNLOCK_THE_DELETE_A_REWARD_BUTTON).ok();
        app::handle_main(events::UNLOCK_THE_EDIT_A_REWARD_BUTTON).ok();
        let index = *selected.borrow() as usize - 1;
        if let Some(rb_i) = items.borrow()[index].find(')') {
            if let Ok(price) = items.borrow()[index][1..rb_i].parse::<f64>() {
                s_price.try_send(price).ok();
                app::handle_main(events::CHECK_AFFORDABILITY_RECEIVE_PRICE).ok();
            }
        }
    }
}

fn handle_custom_events(
    channels: &Channels,
) -> impl Fn(&mut Scroll, &Selected, &Items, i32) -> bool {
    let s_re = channels.rewards_edit.s.clone();
    let s_price = channels.rewards_receive_coins.s.clone();
    let s_item = channels.rewards_receive_item.s.clone();
    let r_item = channels.rewards_send_item.r.clone();
    move |s, selected, items, bits| match bits {
        events::ADD_A_REWARD_RECEIVE => r_item.try_recv().map_or(false, |item| {
            items.borrow_mut().push(item);
            s.resize_list(items.borrow().len() as i32);
            true
        }),
        events::DELETE_A_REWARD => {
            if *selected.borrow() > 0 {
                let index = *selected.borrow() as usize - 1;
                if *selected.borrow() == items.borrow().len() as i32 {
                    *selected.borrow_mut() -= 1;
                }
                if items.borrow().len() == 1 {
                    app::handle_main(events::LOCK_THE_DELETE_A_REWARD_BUTTON).ok();
                    app::handle_main(events::LOCK_THE_EDIT_A_REWARD_BUTTON).ok();
                    app::handle_main(events::LOCK_THE_SPEND_BUTTON).ok();
                } else {
                    handle_selection_closure(selected, items, &s_price);
                }
                items.borrow_mut().remove(index);
                s.resize_list(items.borrow().len() as i32);
            }
            true
        }
        events::EDIT_A_REWARD_RECEIVE => r_item.try_recv().map_or(false, |item| {
            items.borrow_mut()[*selected.borrow() as usize - 1] = item;
            s.resize_list(items.borrow().len() as i32);
            true
        }),
        events::EDIT_A_REWARD_SEND_ITEM => {
            if *selected.borrow() > 0 {
                let string = items.borrow()[*selected.borrow() as usize - 1].clone();
                s_item.try_send(string).ok();
                s_re.try_send(events::EDIT_A_REWARD_OPEN).ok();
                s_re.try_send(events::OK_BUTTON_SET_TO_EDIT).ok();
            }
            true
        }
        events::SPEND_COINS_SEND_PRICE => {
            if *selected.borrow() > 0 {
                let index = *selected.borrow() as usize - 1;
                let rb_i = items.borrow()[index].find(')').unwrap_or_default();
                let price = items.borrow()[index][1..rb_i]
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
