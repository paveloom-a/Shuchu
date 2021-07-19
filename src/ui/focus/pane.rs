use fltk::{
    app,
    enums::{Event, FrameType, Key},
    group::Group,
    prelude::*,
};

use super::{arrow, coins, timer};
use crate::channels::Channels;
use crate::ui::app::CONSTANTS;

pub fn pane(channels: &Channels) -> Group {
    let mut p = Group::default().with_pos(10, 10);
    p.set_frame(FrameType::BorderBox);

    if let Some(ref parent) = p.parent() {
        p.set_size(parent.width() - 20, CONSTANTS.focus_pane_height);
    }

    // The order of the widgets is important
    let _timer = timer();
    let _coins = coins(channels);
    let _arrow = arrow();

    p.end();

    p.handle(handle);

    p
}

fn handle<T: WidgetExt>(p: &mut T, ev: Event) -> bool {
    match ev {
        Event::KeyDown => {
            if app::event_key() == Key::Tab {
                if let Some(ref w) = p.parent() {
                    if let Some(ref mut re_p) = w.child(1) {
                        if let Some(ref mut ra_p) = w.child(2) {
                            if re_p.visible() {
                                re_p.take_focus().ok();
                            } else if ra_p.visible() {
                                ra_p.take_focus().ok();
                            }
                        }
                    }
                }
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
