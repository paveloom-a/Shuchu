use fltk::prelude::*;

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
        let r_reward = channels.rewards_reward.r.clone();
        move |s, items, bits| match bits {
            events::ADD_A_REWARD_RECEIVE => r_reward.try_recv().map_or(false, |reward| {
                items.push(reward);
                s.expand(items.len() as i32);
                true
            }),
            _ => false,
        }
    });
}
