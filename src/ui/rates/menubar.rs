//! Rates' Menubar provides ways to add / delete / edit the conversion rates.

use fltk::{
    app,
    button::Button,
    enums::{Event, Key},
    group::{Pack, PackType},
    image::SvgImage,
    prelude::*,
};

use crate::events;
use crate::ui::constants::{
    DOWN_BUTTON_FRAME_TYPE, FLAT_BUTTON_FRAME_TYPE, REWARDS_MENUBAR_HEIGHT,
};
use crate::ui::logic;

/// Initialize the menubar
pub fn menubar() -> Pack {
    // The menubar is a horizontal pack instead of an
    // actual menubar just for the customization's sake
    let mut m = Pack::default().with_size(0, REWARDS_MENUBAR_HEIGHT);
    m.set_type(PackType::Horizontal);

    // 1. Add a Rate
    let _ab = add_button();
    // 2. Delete the Rate
    let _db = delete_button();

    m.end();

    menubar_logic(&mut m);

    m
}

/// Set a handler for the menubar
fn menubar_logic<T: WidgetBase>(m: &mut T) {
    m.handle(|m, ev| match ev {
        // In case of a pressed down button
        Event::KeyDown => {
            // If it's a `Tab`
            if app::event_key() == Key::Tab {
                // Focus on the Rates' list
                if let Some(ref p) = m.parent() {
                    if let Some(ref mut l) = p.child(1) {
                        l.take_focus().ok();
                    }
                }
                true
            } else {
                false
            }
        }
        _ => false,
    });
}

/// Initialize the add button
fn add_button() -> Button {
    let mut ai = SvgImage::from_data(include_str!("../../../assets/menu/plus.svg")).unwrap();
    ai.scale(24, 24, true, true);

    let mut ab = Button::default().with_size(REWARDS_MENUBAR_HEIGHT, 0);
    ab.set_image(Some(ai));
    ab.set_frame(FLAT_BUTTON_FRAME_TYPE);
    ab.set_down_frame(DOWN_BUTTON_FRAME_TYPE);
    ab.set_tooltip("Add a Rate");

    add_button_logic(&mut ab);

    ab
}

/// Set a callback and a handler for the add button
fn add_button_logic<T: ButtonExt + WidgetBase + 'static>(ab: &mut T) {
    ab.set_callback(|_| {
        println!("Add Pressed!");
    });
    // Handle the shortcut and focus / selection events
    ab.handle(|ab, ev| {
        logic::handle_shortcut(ab, ev.bits(), events::ADD_BUTTON_SHORTCUT)
            || logic::handle_button(ab, ev, 0, false)
    });
}

/// Initialize the delete button
fn delete_button() -> Button {
    let mut di = SvgImage::from_data(include_str!("../../../assets/menu/minus.svg")).unwrap();
    di.scale(24, 24, true, true);

    let mut db = Button::default().with_size(REWARDS_MENUBAR_HEIGHT, 0);
    db.set_image(Some(di));
    db.set_frame(FLAT_BUTTON_FRAME_TYPE);
    db.set_down_frame(DOWN_BUTTON_FRAME_TYPE);
    db.set_tooltip("Delete the Rate");

    delete_button_logic(&mut db);

    db
}

/// Set a callback and a handler for the delete button
fn delete_button_logic<T: ButtonExt + WidgetBase + 'static>(db: &mut T) {
    db.set_callback(|_| {
        println!("Delete Pressed!");
    });
    // Handle the shortcut and focus / selection events
    db.handle(|db, ev| {
        logic::handle_shortcut(db, ev.bits(), events::DELETE_BUTTON_SHORTCUT)
            || logic::handle_button(db, ev, 1, false)
    });
}
