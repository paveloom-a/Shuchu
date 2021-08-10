//! Main Window is the window the user sees after launching the program.

use fltk::{
    app,
    enums::{Event, Key},
    prelude::*,
    window::Window,
};

use super::icon;
use crate::channels::Channels;
use crate::events;
use crate::ui::{
    constants::{FOCUS_PANE_HEIGHT, MAIN_WINDOW_HEIGHT, MAIN_WINDOW_WIDTH},
    focus, rates, rewards,
};

/// Initialize the Main Window
pub fn main(channels: &Channels) -> Window {
    let mut w = Window::new(100, 100, MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT, "Shuchu");

    // Set the default size range
    expand(&mut w);

    w.set_icon(Some(icon()));

    // 1. Focus Pane
    let _fp = focus::pane(channels);

    // 2. Rewards Pane
    let re_p = rewards::pane(channels);

    // 3. Conversion Rates Pane
    let _ra_p = rates::pane();

    w.resizable(&re_p);
    w.end();

    logic(&mut w);

    w.show();
    w
}

/// Set a handle for the window
fn logic<T: WindowExt + WidgetBase>(w: &mut T) {
    w.handle(|w, ev| match ev {
        // Handle shortcuts. This is done manually instead of using the `set_shortcut`
        // method because this way associated buttons will not take focus
        Event::KeyUp => {
            if app::event_key() == Key::from_char('f') {
                app::handle_main(events::TIMER_SHORTCUT).ok();
                true
            } else if app::event_key() == Key::from_char('r') {
                app::handle_main(events::RATES_SHORTCUT).ok();
                true
            } else if app::event_key() == Key::from_char('c') {
                app::handle_main(events::REWARDS_SHORTCUT).ok();
                true
            } else if app::event_key() == Key::from_char('a') {
                app::handle_main(events::ADD_BUTTON_SHORTCUT).ok();
                true
            } else if app::event_key() == Key::from_char('d') {
                app::handle_main(events::DELETE_BUTTON_SHORTCUT).ok();
                true
            } else if app::event_key() == Key::from_char('e') {
                app::handle_main(events::EDIT_BUTTON_SHORTCUT).ok();
                true
            } else if app::event_key() == Key::from_char('s') {
                app::handle_main(events::SPEND_BUTTON_SHORTCUT).ok();
                true
            } else {
                false
            }
        }
        // In case of the
        _ => match ev.bits() {
            // 'Main window: hide the pane' event
            events::MAIN_WINDOW_HIDE_THE_PANE => {
                // Shrink the window
                w.resize(w.x(), w.y(), w.w(), 10 + FOCUS_PANE_HEIGHT + 10);
                // Shrink the size range
                shrink(w);
                true
            }
            events::MAIN_WINDOW_SHOW_THE_PANE => {
                // Expand the window
                w.resize(w.x(), w.y(), w.w(), MAIN_WINDOW_HEIGHT);
                // Expand the size range
                expand(w);
                true
            }
            _ => false,
        },
    });
}

/// Shrink the size range of the window
fn shrink<T: WindowExt>(w: &mut T) {
    w.size_range(
        MAIN_WINDOW_WIDTH,
        10 + FOCUS_PANE_HEIGHT + 10,
        MAIN_WINDOW_WIDTH,
        10 + FOCUS_PANE_HEIGHT + 10,
    );
}

/// Expand the size range of the window
fn expand<T: WindowExt>(w: &mut T) {
    w.size_range(
        MAIN_WINDOW_WIDTH,
        MAIN_WINDOW_HEIGHT,
        MAIN_WINDOW_WIDTH,
        MAIN_WINDOW_HEIGHT + 100,
    );
}
