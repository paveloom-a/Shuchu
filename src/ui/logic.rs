use fltk::{
    app,
    enums::{Color, Event, FrameType, Key},
    prelude::*,
};

/// Handle focus / selection events
pub fn handle_selection<T: ButtonExt>(b: &mut T, ev: Event) -> bool {
    match ev {
        Event::Focus => {
            if app::event_key_down(Key::Enter) {
                select(b);
            }
            true
        }
        Event::Enter => {
            select(b);
            true
        }
        Event::Leave | Event::Unfocus => {
            unselect(b);
            true
        }
        Event::KeyDown => match app::event_key() {
            Key::Enter => {
                select(b);
                true
            }
            _ => false,
        },
        Event::KeyUp => match app::event_key() {
            Key::Enter => {
                b.do_callback();
                unselect(b);
                true
            }
            _ => false,
        },
        _ => false,
    }
}

fn select<T: ButtonExt>(b: &mut T) {
    b.set_color(Color::from_hex(0xE5_F3_FF));
    b.set_frame(FrameType::BorderBox);
    b.set_selection_color(Color::from_hex(0xE5_F3_FF));
    b.set_down_frame(FrameType::DownBox);
    b.redraw();
}

fn unselect<T: ButtonExt>(b: &mut T) {
    b.set_color(Color::BackGround);
    b.set_frame(FrameType::FlatBox);
    b.set_selection_color(Color::BackGround);
    b.set_down_frame(FrameType::FlatBox);
    b.redraw();
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
