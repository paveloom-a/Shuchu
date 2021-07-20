use fltk::{
    app,
    enums::{Event, Key},
    group::{Pack, PackType},
    prelude::*,
};

use super::{add_button, delete_button, divider, edit_button, spend_button};
use crate::channels::Channels;
use crate::ui::app::CONSTANTS;

pub fn new(channels: &Channels) -> Pack {
    let mut m = Pack::default().with_size(0, CONSTANTS.rewards_menubar_height);
    m.set_type(PackType::Horizontal);

    // 1. Add a Reward
    let _ab = add_button(channels);

    // 2. Delete the Reward
    let _db = delete_button();

    // 3. Edit the Reward
    let _eb = edit_button();

    // 4. Divider
    let _d = divider();

    // 5. Spend Coins for the Reward
    let _sb = spend_button();

    m.end();

    logic(&mut m);

    m
}

fn logic<T: WidgetBase>(m: &mut T) {
    m.handle(|m, ev| match ev {
        Event::KeyDown => {
            if app::event_key() == Key::Tab {
                if let Some(ref p) = m.parent() {
                    if let Some(ref mut l) = p.child(1) {
                        l.take_focus().ok();
                    }
                }
                true
            } else {
                false
            }
        }
        _ => false,
    });
}
