use fltk::{prelude::*, window::Window};

use super::icon;
use crate::channels::Channels;
use crate::events;
use crate::ui::{app::CONSTANTS, focus, rates, rewards};

/// Create the Main Window
pub fn main(channels: &Channels) -> Window {
    let mut w = Window::new(
        100,
        100,
        CONSTANTS.main_window_width,
        CONSTANTS.main_window_height,
        "Shuchu",
    );
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

fn logic<T: WindowExt + WidgetBase>(w: &mut T) {
    w.handle(|w, ev| match ev.bits() {
        events::MAIN_WINDOW_HIDE_THE_PANE => {
            w.resize(w.x(), w.y(), w.w(), 10 + CONSTANTS.focus_pane_height + 10);
            shrink(w);
            true
        }
        events::MAIN_WINDOW_SHOW_THE_PANE => {
            w.resize(w.x(), w.y(), w.w(), CONSTANTS.main_window_height);
            expand(w);
            true
        }
        _ => false,
    });
}

fn shrink<T: WindowExt>(w: &mut T) {
    w.size_range(
        CONSTANTS.main_window_width,
        10 + CONSTANTS.focus_pane_height + 10,
        CONSTANTS.main_window_width,
        10 + CONSTANTS.focus_pane_height + 10,
    );
}

fn expand<T: WindowExt>(w: &mut T) {
    w.size_range(
        CONSTANTS.main_window_width,
        CONSTANTS.main_window_height,
        CONSTANTS.main_window_width,
        CONSTANTS.main_window_height + 100,
    );
}
