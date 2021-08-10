//! The pane itself. It is supposed to be the top-most pane in the main window.

use fltk::{
    app,
    enums::{Event, FrameType, Key},
    group::Group,
    prelude::*,
};

use super::{arrow, coins, timer};
use crate::channels::Channels;
use crate::ui::constants::FOCUS_PANE_HEIGHT;

/// Initialize the Focus pane
pub fn pane(channels: &Channels) -> Group {
    let mut p = Group::default().with_pos(10, 10);
    p.set_frame(FrameType::BorderBox);

    // If this pane is a child of the Main Window
    if let Some(ref w) = p.parent() {
        // Set the size, excluding the margins
        p.set_size(w.width() - 20, FOCUS_PANE_HEIGHT);
    }

    // Initialize the widgets, so that the `arrow` widget is the last one
    let _timer = timer();
    let coins = coins(channels);
    let arrow = arrow();

    // The reason why the pack ends here and removes / adds the items later is that
    // the `arrow` widget needs to know about the positions and sizes of the
    // other two widgets: `timer` and `coins`, because the `coins`' size depends on
    // the number of coins, and the `arrow` widget is supposed to be placed between
    // the two. But it is also important to have the `arrow` widget as a second
    // child, so it is possible to use the default handle function for the buttons
    p.end();

    // Reorder the children, so that the `arrow` widget is the second one
    p.remove(&arrow);
    p.remove(&coins);
    p.add(&arrow);
    p.add(&coins);

    p.handle(handle);

    p
}

/// Handle events for the Focus pane
fn handle<T: WidgetExt>(p: &mut T, ev: Event) -> bool {
    match ev {
        // In case of a pressed down button
        Event::KeyDown => {
            // If it's a `Tab` key
            if app::event_key() == Key::Tab {
                // Focus on the secondary pane
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
