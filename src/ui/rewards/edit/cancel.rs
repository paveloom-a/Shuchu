use fltk::{button::Button, enums::FrameType, prelude::*};

use crate::ui::logic;

pub fn cancel() -> Button {
    let mut c = Button::default().with_size(80, 0).with_label("Cancel");
    c.set_down_frame(FrameType::DownBox);

    logic(&mut c);

    c
}

fn logic<T: WidgetBase + ButtonExt>(b: &mut T) {
    b.set_callback(|b| {
        if let Some(ref p) = b.parent() {
            if let Some(ref mut w) = p.parent() {
                w.hide();
            }
        }
    });
    b.handle(|c, ev| logic::handle_active_selection(c, ev, FrameType::BorderBox));
}
