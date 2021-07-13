use fltk::{
    app,
    enums::{Color, Event, FrameType, Key},
    prelude::*,
};

pub fn button_handle<T: ButtonExt>(b: &mut T, ev: Event) -> bool {
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
