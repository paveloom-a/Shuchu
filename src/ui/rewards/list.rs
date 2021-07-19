use fltk::{app, prelude::*};

use crate::channels::Channels;
use crate::events;
use crate::ui::widgets::{List, ScrollExt};

pub fn list(channels: &Channels) -> List {
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
    l.handle({
        let s_re = channels.rewards_edit.s.clone();
        let s_diff = channels.rewards_receive_coins.s.clone();
        let s_item = channels.rewards_receive_item.s.clone();
        let r_coins = channels.rewards_send_coins.r.clone();
        let r_item = channels.rewards_send_item.r.clone();
        move |s, selected, items, bits| match bits {
            events::ADD_A_REWARD_RECEIVE => r_item.try_recv().map_or(false, |item| {
                items.push(item);
                s.resize_list(items.len() as i32);
                true
            }),
            events::DELETE_A_REWARD => {
                if *selected != 0 {
                    let index = *selected as usize - 1;
                    if *selected == items.len() as i32 {
                        *selected -= 1;
                    }
                    items.remove(index);
                    s.resize_list(items.len() as i32);
                }
                true
            }
            events::EDIT_A_REWARD_RECEIVE => r_item.try_recv().map_or(false, |item| {
                items[*selected as usize - 1] = item;
                s.resize_list(items.len() as i32);
                true
            }),
            events::EDIT_A_REWARD_SEND_ITEM => {
                if *selected != 0 {
                    let string = items[*selected as usize - 1].clone();
                    s_item.try_send(string).ok();
                    s_re.try_send(events::EDIT_A_REWARD_OPEN).ok();
                    s_re.try_send(events::OK_BUTTON_SET_TO_EDIT).ok();
                }
                true
            }
            events::SPEND_COINS_RECEIVE_TOTAL => {
                if *selected != 0 {
                    let index = *selected as usize - 1;
                    if let Ok(coins) = r_coins.try_recv() {
                        if let Some(rb_i) = items[index].find(')') {
                            if let Ok(price) = items[index][1..rb_i].parse::<f64>() {
                                if price <= coins {
                                    if *selected == items.len() as i32 {
                                        *selected -= 1;
                                    }
                                    items.remove(index);
                                    s.resize_list(items.len() as i32);

                                    let diff = coins - price;
                                    s_diff.try_send((diff * 100.0).round() / 100.0).ok();
                                    app::handle_main(events::SPEND_COINS_RECEIVE_DIFF).ok();
                                } else {
                                    println!("Not enough coins!");
                                }
                            }
                        }
                    }
                }
                true
            }
            _ => false,
        }
    });
}
