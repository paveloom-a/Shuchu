//! Handle [`Focus`](fltk::enums::Event::Focus) and other selection events.

use fltk::{
    app,
    enums::{Color, Event, FrameType, Key},
    prelude::*,
};

/// Color of an active button
const ACTIVE_SELECTION_COLOR: u32 = 0xE5_F3_FF;
/// Color of an inactive button
const INACTIVE_SELECTION_COLOR: u32 = 0xF5_FA_FE;

/// Handle focus / selection events for the active button
fn handle_active_selection<T: ButtonExt>(
    b: &mut T,
    ev: Event,
    unselect_frame_type: FrameType,
) -> bool {
    match ev {
        Event::Focus => {
            if app::event_key_down(Key::Enter) {
                select_active(b);
            }
            true
        }
        Event::Enter => {
            select_active(b);
            true
        }
        Event::Leave | Event::Unfocus | Event::Hide => {
            unselect(b, unselect_frame_type);
            true
        }
        Event::KeyDown => match app::event_key() {
            Key::Enter => {
                select_active(b);
                true
            }
            _ => false,
        },
        Event::KeyUp => match app::event_key() {
            Key::Enter => {
                b.do_callback();
                unselect(b, unselect_frame_type);
                true
            }
            _ => false,
        },
        _ => false,
    }
}

/// Handle focus / selection events for the inactive button
fn handle_inactive_selection<T: ButtonExt>(
    b: &mut T,
    ev: Event,
    unselect_frame_type: FrameType,
) -> bool {
    match ev {
        Event::Focus => {
            if app::event_key_down(Key::Enter) {
                select_inactive(b);
            }
            true
        }
        Event::Enter => {
            select_inactive(b);
            true
        }
        Event::Leave | Event::Unfocus | Event::Hide => {
            unselect(b, unselect_frame_type);
            true
        }
        Event::KeyDown => match app::event_key() {
            Key::Enter => {
                select_inactive(b);
                true
            }
            _ => false,
        },
        Event::KeyUp => match app::event_key() {
            Key::Enter => {
                unselect(b, unselect_frame_type);
                true
            }
            _ => false,
        },
        _ => false,
    }
}

/// Handle focus / selection events for the button
pub fn handle_selection<T: ButtonExt>(
    b: &mut T,
    ev: Event,
    unselect_frame_type: FrameType,
    lock: bool,
) -> bool {
    if lock {
        handle_inactive_selection(b, ev, unselect_frame_type)
    } else {
        handle_active_selection(b, ev, unselect_frame_type)
    }
}

/// Select the button with the specified color
fn select<T: ButtonExt>(b: &mut T, hex: u32) {
    b.set_color(Color::from_hex(hex));
    b.set_frame(FrameType::BorderBox);
    b.redraw();
}

/// Select the active button
pub fn select_active<T: ButtonExt>(b: &mut T) {
    select(b, ACTIVE_SELECTION_COLOR);
}

/// Select the inactive button
pub fn select_inactive<T: ButtonExt>(b: &mut T) {
    select(b, INACTIVE_SELECTION_COLOR);
}

/// Unselect the button
fn unselect<T: ButtonExt>(b: &mut T, unselect_frame_type: FrameType) {
    b.set_color(Color::BackGround);
    b.set_frame(unselect_frame_type);
    b.redraw();
}
