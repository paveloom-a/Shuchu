use fltk::{
    app,
    enums::{Event, FrameType, Key},
    group::{Group, Pack, Scroll, ScrollType},
    prelude::*,
};

use super::{Holder, Items, Selected};
use crate::ui::app::CONSTANTS;

pub struct List {
    scroll: Scroll,
    holder: Holder,
    selected: Selected,
    items: Items,
}

impl List {
    pub fn default() -> List {
        let mut scroll = Scroll::default();
        scroll.set_frame(FrameType::BorderBox);
        scroll.set_type(ScrollType::Vertical);
        scroll.set_scrollbar_size(CONSTANTS.scrollbar_width);

        let holder = Holder::default();

        scroll.end();

        let mut w = List {
            scroll,
            holder,
            selected: Selected::default(),
            items: Items::default(),
        };
        w.handle(|_, _, _| {}, |_, _, _| {}, |_, _, _, _| false);
        w
    }

    pub fn scroll(&self) -> &Scroll {
        &self.scroll
    }

    pub fn parent(&self) -> Option<Group> {
        self.scroll.parent()
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        if width == 0 {
            if let Some(p) = self.parent() {
                self.scroll.set_size(p.w(), height);
            }
        } else {
            self.scroll.set_size(width, height);
        }
    }

    pub fn add(&mut self, s: &'static str) {
        self.holder.add(s.to_string(), &self.items);
    }

    pub fn select_in(selected: &Selected, items: &Items, idx: usize) {
        selected.set(idx);
        items.select(idx);
    }

    pub fn select(&self, idx: usize) {
        Self::select_in(&self.selected, &self.items, idx)
    }

    pub fn handle<A: 'static, B: 'static, C: 'static>(
        &mut self,
        handle_push: A,
        handle_key_down: B,
        handle_custom_events: C,
    ) where
        A: Fn(&mut Scroll, &Selected, &Items),
        B: Fn(&mut Scroll, &Selected, &Items),
        C: Fn(&mut Pack, &mut Selected, &mut Items, i32) -> bool,
    {
        self.scroll.handle({
            let mut selected = self.selected.clone();
            let mut items = self.items.clone();
            move |s, ev| match ev {
                Event::Focus => true,
                Event::Push => {
                    handle_push_default(s, &mut selected, &mut items);
                    handle_push(s, &selected, &items);
                    true
                }
                Event::KeyDown => {
                    if handle_key_down_default(s, &mut selected, &mut items) {
                        handle_key_down(s, &selected, &items);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }
        });

        self.holder.handle({
            let mut selected = self.selected.clone();
            let mut items = self.items.clone();
            move |l, ev| handle_custom_events(l, &mut selected, &mut items, ev.bits())
        })
    }
}

fn handle_push_default(s: &mut Scroll, selected: &mut Selected, items: &mut Items) {
    s.take_focus().ok();

    if let Some(l) = s.child(0) {
        // Exclude scrollbar clicks
        if l.h() < s.h() - 20
            || (l.h() >= s.h() - 20 && app::event_x() < s.x() + s.w() - CONSTANTS.scrollbar_width)
        {
            let idx = ((app::event_y() - s.y() + s.yposition()) / 20) as usize + 1;

            if idx < items.len() {
                selected.set(idx);
            } else {
                selected.set(items.len());
            }
            items.select(selected.get());

            s.redraw();
        }
    }
}

/// Set the default handles for the `Key::Up` and `Key::Down` events
fn handle_key_down_default(s: &mut Scroll, selected: &mut Selected, items: &mut Items) -> bool {
    match app::event_key() {
        Key::Up => {
            // Pressing Up without any selection or on the top item will
            let to = if selected.get() == 0 || selected.get() == 1 {
                // Select the last item
                selected.set(items.len());
                // Set the scroll position to the bottom of the list
                // (these two pixels come from the compensation of the Scroll borders)
                selected.get() as i32 * 20 - s.h() + 2
            // Pressing Up on any other item will
            } else {
                // Select the previous item
                selected.decrement();
                // Set the scroll position to the top border of the previous item
                selected.index() as i32 * 20
            };
            // If the selected item is partially or completely hidden, change the scroll position
            if items.hidden(selected.index()) {
                s.scroll_to(0, to);
            }
            // Select the new item, unselecting all others
            items.select(selected.get());

            s.redraw();
            true
        }
        Key::Down => {
            // Pressing Down without on the last item will
            let to = if selected.get() == items.len() {
                // Select the top item
                selected.set(1);
                // Set the scroll position to the top of the list
                0
            } else {
                // Select the next item
                selected.increment();
                // Set the scroll position, so that the current item is on the top
                // (these two pixels come from the compensation of the Scroll borders)
                selected.get() as i32 * 20 - s.h() + 2
            };
            // If the selected item is partially or completely hidden, change the scroll position
            if items.hidden(selected.index()) {
                s.scroll_to(0, to);
            }
            // Select the new item, unselecting all others
            items.select(selected.get());

            s.redraw();
            true
        }
        _ => false,
    }
}
