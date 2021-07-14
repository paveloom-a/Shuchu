use fltk::{
    app,
    browser::{BrowserType, SelectBrowser},
    enums::{Event, FrameType, Key},
    prelude::*,
};

pub fn list() -> SelectBrowser {
    let mut list = SelectBrowser::default();
    list.set_frame(FrameType::BorderBox);
    list.set_type(BrowserType::Hold);
    list.set_text_size(16);

    if let Some(ref p) = list.parent() {
        list.set_size(0, p.h() - 20);
    }

    list.add("5/m");
    list.add("0.1/s");
    list.select(1);

    logic(&mut list);

    list
}

fn logic<T: WidgetBase + BrowserExt>(l: &mut T) {
    l.handle(|l, ev| match ev {
        Event::Focus => true,
        Event::Released => {
            if l.value() == 0 {
                l.select(l.size());
            }
            true
        }
        Event::KeyDown => match app::event_key() {
            Key::Down => {
                l.set_type(BrowserType::Select);
                let i = l.value();
                if i == l.size() {
                    l.select(1);
                } else {
                    l.select(i + 1);
                }
                true
            }
            Key::Up => {
                l.set_type(BrowserType::Select);
                let i = l.value();
                if i == 1 {
                    l.select(l.size());
                } else {
                    l.select(i - 1);
                }
                true
            }
            _ => false,
        },
        Event::KeyUp => {
            if app::event_key() == Key::Down | Key::Up {
                l.set_type(BrowserType::Hold);
            }
            true
        }
        _ => false,
    });
}
