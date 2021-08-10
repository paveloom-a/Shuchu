//! Handle the events for the button.

use fltk::{
    app,
    enums::{Event, Key},
    prelude::*,
};

use super::{handle_left, handle_right, handle_selection, handle_tab};
use crate::ui::constants::FLAT_BUTTON_FRAME_TYPE;

/// Handle the events for the button
pub fn handle_button<T: ButtonExt>(b: &mut T, ev: Event, idx: i32, lock: bool) -> bool {
    match ev {
        Event::KeyDown => match app::event_key() {
            Key::Left => handle_left(b, idx),
            Key::Right => handle_right(b, idx),
            _ => handle_selection(b, ev, FLAT_BUTTON_FRAME_TYPE, lock),
        },
        _ => handle_selection(b, ev, FLAT_BUTTON_FRAME_TYPE, lock),
    }
}

/// Handle the events for the button in the Focus pane
pub fn handle_fp_button<T: ButtonExt>(b: &mut T, ev: Event, idx: i32) -> bool {
    if ev == Event::KeyDown {
        match app::event_key() {
            Key::Tab => handle_tab(b),
            _ => handle_button(b, ev, idx, false),
        }
    } else {
        handle_button(b, ev, idx, false)
    }
}
