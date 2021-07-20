use fltk::{
    app,
    enums::{Color, Event, FrameType, Key},
    prelude::*,
};

const ACTIVE_SELECTION_COLOR: u32 = 0xE5_F3_FF;
const INACTIVE_SELECTION_COLOR: u32 = 0xF5_FA_FE;

/// Handle focus / selection events
pub fn handle_active_selection<T: ButtonExt>(
    b: &mut T,
    ev: Event,
    unselect_box: FrameType,
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
            unselect(b, unselect_box);
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
                unselect(b, unselect_box);
                true
            }
            _ => false,
        },
        _ => false,
    }
}

pub fn handle_inactive_selection<T: ButtonExt>(
    b: &mut T,
    ev: Event,
    unselect_box: FrameType,
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
            unselect(b, unselect_box);
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
                unselect(b, unselect_box);
                true
            }
            _ => false,
        },
        _ => false,
    }
}

pub fn handle_selection<T: ButtonExt>(
    b: &mut T,
    ev: Event,
    unselect_box: FrameType,
    lock: bool,
) -> bool {
    if lock {
        handle_inactive_selection(b, ev, unselect_box)
    } else {
        handle_active_selection(b, ev, unselect_box)
    }
}

fn select<T: ButtonExt>(b: &mut T, hex: u32) {
    b.set_color(Color::from_hex(hex));
    b.set_frame(FrameType::BorderBox);
    b.redraw();
}

pub fn select_active<T: ButtonExt>(b: &mut T) {
    select(b, ACTIVE_SELECTION_COLOR);
}

pub fn select_inactive<T: ButtonExt>(b: &mut T) {
    select(b, INACTIVE_SELECTION_COLOR);
}

fn unselect<T: ButtonExt>(b: &mut T, unselect_box: FrameType) {
    b.set_color(Color::BackGround);
    b.set_frame(unselect_box);
    b.redraw();
}

pub fn mouse_hovering_widget<T: WidgetBase>(b: &mut T) -> bool {
    app::event_x() >= b.x()
        && app::event_x() <= b.x() + b.w()
        && app::event_y() >= b.y()
        && app::event_y() <= b.y() + b.h()
}

/// Handle Left events for the buttons in the Focus pane
pub fn fp_handle_left<T: ButtonExt>(b: &mut T, idx: i32) -> bool {
    if let Some(ref p) = b.parent() {
        if let Some(ref mut cw) = p.child(if idx == 2 { 0 } else { idx + 1 }) {
            cw.take_focus().ok();
        }
    }
    true
}

/// Handle Right events for the buttons in the Focus pane
pub fn fp_handle_right<T: ButtonExt>(b: &mut T, idx: i32) -> bool {
    if let Some(ref p) = b.parent() {
        if let Some(ref mut cw) = p.child(if idx == 0 { 2 } else { idx - 1 }) {
            cw.take_focus().ok();
        }
    }
    true
}

/// Handle Left events for the buttons in the Rewards / Rates panes
pub fn rp_handle_left<T: ButtonExt>(b: &mut T, idx: i32) -> bool {
    if let Some(ref m) = b.parent() {
        if let Some(ref mut cw) = m.child(if idx == 0 { m.children() - 1 } else { idx - 1 }) {
            cw.take_focus().ok();
        }
    }
    true
}

/// Handle Right events for the buttons in the Rewards / Rates panes
pub fn rp_handle_right<T: ButtonExt>(b: &mut T, idx: i32) -> bool {
    if let Some(ref m) = b.parent() {
        if let Some(ref mut cw) = m.child(if idx == (m.children() - 1) {
            0
        } else {
            idx + 1
        }) {
            cw.take_focus().ok();
        }
    }
    true
}

/// Handle Tab events for the buttons in the Focus Pane
pub fn handle_tab<T: ButtonExt>(b: &mut T) -> bool {
    b.parent().map_or(false, |p| {
        p.parent().map_or(false, |w| {
            w.child(1).map_or(false, |re_p| {
                w.child(2)
                    .map_or(false, |ra_p| !(re_p.visible() || ra_p.visible()))
            })
        })
    })
}
