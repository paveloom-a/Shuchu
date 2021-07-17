use fltk::{app, button::Button, enums::FrameType, prelude::*};

use crate::events;
use crate::ui::logic;

pub fn cancel() -> Button {
    let mut cancel = Button::default().with_size(80, 0).with_label("Cancel");

    logic(&mut cancel);

    cancel
}

fn logic<T: WidgetBase + ButtonExt>(b: &mut T) {
    b.set_callback(|b| {
        app::handle_main(events::ADD_A_REWARD_RESET_COINS).ok();
        app::handle_main(events::ADD_A_REWARD_RESET_REWARD).ok();
        if let Some(ref p) = b.parent() {
            if let Some(ref mut w) = p.parent() {
                w.hide();
            }
        }
    });
    b.handle(|c, ev| logic::handle_selection(c, ev, FrameType::BorderBox));
}
