//! The add button opens the [Rewards Edit](super::super::edit) window, set to add a new item.

use fltk::{button::Button, image::SvgImage, prelude::*};

use crate::channels::Channels;
use crate::events;
use crate::ui::constants::{
    DOWN_BUTTON_FRAME_TYPE, FLAT_BUTTON_FRAME_TYPE, REWARDS_MENUBAR_HEIGHT,
};
use crate::ui::logic;

/// Initialize the add button
pub fn add_button(channels: &Channels) -> Button {
    let mut ai = SvgImage::from_data(include_str!("../../../../assets/menu/plus.svg")).unwrap();
    ai.scale(24, 24, true, true);

    let mut ab = Button::default().with_size(REWARDS_MENUBAR_HEIGHT, 0);
    ab.set_image(Some(ai));
    ab.set_frame(FLAT_BUTTON_FRAME_TYPE);
    ab.set_down_frame(DOWN_BUTTON_FRAME_TYPE);
    ab.set_tooltip("Add a Reward");

    logic(&mut ab, channels);

    ab
}

/// Set a callback and a handler for the add button
fn logic<T: ButtonExt + WidgetBase + 'static>(ab: &mut T, channels: &Channels) {
    // Open the Rewards Edit window on push, set to add a new reward
    ab.set_callback({
        // Send signals to the Rewards Edit window
        let s = channels.rewards_edit.s.clone();
        move |_| {
            s.try_send(events::ADD_A_REWARD_OPEN).ok();
        }
    });
    // Handle the shortcut and focus / selection events
    ab.handle(|ab, ev| {
        logic::handle_shortcut(ab, ev.bits(), events::ADD_BUTTON_SHORTCUT)
            || logic::handle_button(ab, ev, 0, false)
    });
}
