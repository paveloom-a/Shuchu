use fltk::{app, button::Button, enums::FrameType, prelude::*};

use crate::events;
use crate::ui::logic;

pub fn ok() -> Button {
    let mut ok = Button::default().with_size(80, 0).with_label("OK");

    logic(&mut ok);

    ok
}

fn logic<T: WidgetBase + ButtonExt>(b: &mut T) {
    b.set_callback(|b| {
        app::handle_main(events::ADD_A_REWARD_SEND_COINS).ok();
        if let Some(ref p) = b.parent() {
            if let Some(ref mut w) = p.parent() {
                w.hide();
            }
        }
    });
    b.handle(|o, ev| logic::handle_selection(o, ev, FrameType::BorderBox));
}
