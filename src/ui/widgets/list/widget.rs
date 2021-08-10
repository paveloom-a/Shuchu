//! The widget itself.
//!
//! Visually it consists of a packed into a vertical [`Scroll`](Scroll)
//! [`Holder`](Holder), which handles the items as a collection of [`Frame`](fltk::frame::Frame)s.
//! The items itself are stored in the [`Items`] vector.

use fltk::{
    app,
    enums::{Event, FrameType, Key},
    group::{Group, Scroll, ScrollType},
    prelude::*,
};

use super::{Holder, Items, Selected, ITEM_HEIGHT, SCROLLBAR_WIDTH};

/// A [`Browser`](fltk::browser::Browser) replica that provides more customization
pub struct List {
    /// Vertical scroll
    scroll: Scroll,
    /// Items holder
    holder: Holder,
    /// Index of the selected item
    selected: Selected,
    /// Vector of the items
    items: Items,
}

impl List {
    /// Get the default struct
    pub fn default() -> List {
        // The Scroll will be visible as a bounding box
        let mut scroll = Scroll::default();
        scroll.set_frame(FrameType::BorderBox);
        scroll.set_type(ScrollType::Vertical);
        scroll.set_scrollbar_size(SCROLLBAR_WIDTH);

        // Create the Holder as a child of the Scroll. Mind the 1px borders of the Scroll
        let holder = Holder::default().with_pos(scroll.x() + 1, scroll.y() + 1);

        scroll.end();

        let mut l = List {
            scroll,
            holder,
            selected: Selected::default(),
            items: Items::default(),
        };

        // Set the default handles (implicitly), providing empty custom handles (explicitly)
        l.handle(|_, _, _| {}, |_, _, _| {}, |_, _, _, _, _| false);

        l
    }

    /// Get the Scroll
    pub fn scroll(&self) -> &Scroll {
        &self.scroll
    }

    /// Get the parent of the Scroll
    pub fn parent(&self) -> Option<Group> {
        self.scroll.parent()
    }

    /// Set the size of the widget
    pub fn set_size(&mut self, width: i32, height: i32) {
        // If the specified width is equal to 0
        if width == 0 {
            // Imitate the standard behavior for the packs: inherit the width of the parent
            if let Some(p) = self.parent() {
                self.scroll.set_size(p.w(), height);
                // Also, mind the borders of the Scroll
                self.holder.set_size(p.w() - 2, self.holder.h());
            }
        // Otherwise,
        } else {
            // Set the size to the specified values
            self.scroll.set_size(width, height);
            // Also, mind the borders of the Scroll
            self.holder.set_size(width - 2, self.holder.h());
        }
    }

    /// Add an item to the List
    pub fn add(&mut self, s: &'static str) {
        self.holder
            .add(s.to_string(), &mut self.scroll, &self.items);
    }

    /// Select an item at the specified index (explicitly; useful in the handles)
    pub fn select_in(selected: &Selected, items: &Items, idx: usize) {
        selected.set(idx);
        items.select(idx);
    }

    /// Select an item at the specified index
    pub fn select(&self, idx: usize) {
        Self::select_in(&self.selected, &self.items, idx);
    }

    /// Apply the default handles and set the custom ones
    pub fn handle<A: 'static, B: 'static, C: 'static>(
        &mut self,
        handle_push: A,
        handle_key_down: B,
        handle_custom_events: C,
    ) where
        A: Fn(&mut Scroll, &Selected, &Items),
        B: Fn(&mut Scroll, &Selected, &Items),
        C: Fn(&mut Scroll, &Holder, &Selected, &Items, i32) -> bool,
    {
        self.scroll.handle({
            let mut handle = self.holder.clone();
            let mut selected = self.selected.clone();
            let mut items = self.items.clone();
            move |s, ev| match ev {
                // Setting `true` here will allow to focus on the List
                Event::Focus => true,
                // Apply default and custom handles for the `Push` event
                Event::Push => {
                    handle_push_default(s, &mut selected, &mut items);
                    handle_push(s, &selected, &items);
                    true
                }
                // Apply the default and custom handles for the `KeyDown` event
                Event::KeyDown => {
                    // If the default `KeyDown` buttons were handled
                    if handle_key_down_default(s, &mut selected, &mut items) {
                        // Handle the custom `KeyDown` events for these buttons
                        handle_key_down(s, &selected, &items);
                        true
                    } else {
                        false
                    }
                }
                // Block the `Enter` button from doing anything on the `KeyUp` event
                Event::KeyUp => app::event_key() == Key::Enter,
                // Handle custom events (that is, signals)
                _ => handle_custom_events(s, &mut handle, &mut selected, &mut items, ev.bits()),
            }
        });
    }
}

/// Set the default handle for the [`Push`](Event::Push) event
fn handle_push_default(s: &mut Scroll, selected: &mut Selected, items: &mut Items) {
    s.take_focus().ok();

    // Get the Holder
    if let Some(h) = s.child(0) {
        // If the click happened on an item (that is, if there is a scrollbar, exclude its width)
        if h.h() < s.h() - ITEM_HEIGHT
            || (h.h() >= s.h() - ITEM_HEIGHT && app::event_x() < s.x() + s.w() - SCROLLBAR_WIDTH)
        {
            // Compute the index of the clicked item
            let idx = ((app::event_y() - s.y() + s.yposition()) / ITEM_HEIGHT) as usize + 1;

            // If this index is inside the bounds
            if idx < items.len() {
                // Select an item at that index
                selected.set(idx);
            // Otherwise (that is, if clicking at the empty space below the list),
            } else {
                // Select the last item
                selected.set(items.len());
            }

            // Apply the selection
            items.select(selected.get());

            s.redraw();
        }
    }
}

/// Set the default handles for the [`Event::KeyDown`] events
fn handle_key_down_default(s: &mut Scroll, selected: &mut Selected, items: &mut Items) -> bool {
    match app::event_key() {
        // Pressing Up
        Key::Up => {
            // Without any selection or on the top item will
            let to = if selected.get() == 0 || selected.get() == 1 {
                // Select the last item
                selected.set(items.len());
                // Set the scroll position to the bottom of the list
                // (these two pixels come from the compensation of the Scroll borders)
                selected.get() as i32 * ITEM_HEIGHT - s.h() + 2
            // On any other item will
            } else {
                // Select the previous item
                selected.decrement();
                // If the selected item is close to the bottom of the list (meaning, if the
                // scroll is at the bottom of the list, this item would be visible)
                if (selected.get() as i32) > (items.len() as i32) - s.h() / ITEM_HEIGHT {
                    // Set the scroll position to the bottom of the list
                    // (these two pixels come from the compensation of the Scroll borders)
                    items.len() as i32 * ITEM_HEIGHT - s.h() + 2
                // Otherwise,
                } else {
                    // Set the scroll position to the top border of the previous item
                    selected.index() as i32 * ITEM_HEIGHT
                }
            };
            // If the selected item is partially or completely hidden, change the scroll position
            if items.index(selected.index()).hidden(false) {
                s.scroll_to(0, to);
            }
            // Select the new item, unselecting all others
            items.select(selected.get());

            s.redraw();
            true
        }
        // Pressing Down
        Key::Down => {
            // Without any selection or on the last item will
            let to = if selected.get() == items.len() {
                // Select the top item
                selected.set(1);
                // Set the scroll position to the top of the list
                0
            // On any other item will
            } else {
                // Select the next item
                selected.increment();
                // If the selected item is close to the top of the list (meaning, if the
                // scroll is at the top of the list, this item would be visible)
                if (selected.get() as i32) <= s.h() / ITEM_HEIGHT {
                    // Set the scroll position to the top of the list
                    0
                // Otherwise,
                } else {
                    // Set the scroll position, so that the current item is at the bottom
                    // (these two pixels come from the compensation of the Scroll borders)
                    selected.get() as i32 * ITEM_HEIGHT - s.h() + 2
                }
            };
            // If the selected item is partially or completely hidden, change the scroll position
            if items.index(selected.index()).hidden(false) {
                s.scroll_to(0, to);
            }
            // Select the new item, unselecting all others
            items.select(selected.get());

            s.redraw();
            true
        }
        Key::Enter => true,
        _ => false,
    }
}
