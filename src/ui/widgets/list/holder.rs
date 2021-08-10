//! List's Holder is a wrapper around a [`Pack`](fltk::group::Pack) that manages
//! [`Items`](super::Items).

use fltk::{
    group::{Pack, Scroll},
    prelude::*,
};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use super::{item::Item, Items, ITEM_HEIGHT};

/// A wrapper around a [`Pack`](fltk::group::Pack)
pub struct Holder {
    /// A reference-counting pointer to a mutable [`Pack`](fltk::group::Pack)
    holder: Rc<RefCell<Pack>>,
}

impl Holder {
    /// Get the default struct
    pub fn default() -> Self {
        Holder {
            holder: Rc::<RefCell<Pack>>::default(),
        }
    }

    /// Get an immutable reference to the pack
    fn holder(&self) -> Ref<Pack> {
        self.holder.borrow()
    }

    /// Get a mutable reference to the pack
    fn holder_mut(&self) -> RefMut<Pack> {
        self.holder.borrow_mut()
    }

    /// Get the height of the pack
    pub fn h(&self) -> i32 {
        self.holder().h()
    }

    /// Set the position of the pack and return it back
    pub fn with_pos(self, x: i32, y: i32) -> Self {
        self.holder_mut().set_pos(x, y);
        self
    }

    /// Set the size of the pack
    pub fn set_size(&self, width: i32, height: i32) {
        self.holder_mut().set_size(width, height);
    }

    /// Resize the pack's height to `ITEM_HEIGHT * n`
    fn resize(&self, n: usize) {
        let w = self.holder().w();
        self.holder_mut().set_size(w, n as i32 * ITEM_HEIGHT);
    }

    /// Redraw the pack
    pub fn redraw(&self) {
        self.holder_mut().redraw();
    }

    /// Add an item to the List
    pub fn add(&self, string: String, scroll: &mut Scroll, items: &Items) {
        // Resize the Holder (expand it)
        self.resize(items.len() + 1);

        // Create an Item as a child of the Holder
        self.holder().begin();
        let item = Item::new(string);
        self.holder().end();

        items.push(item);

        // Reset the scroll position to where it was before, redraw the Scroll
        scroll.scroll_to(0, scroll.yposition());
        scroll.redraw();
    }

    /// Remove an item from the List
    pub fn remove(&self, index: usize, scroll: &mut Scroll, items: &Items) {
        // Determine the next scroll position:
        // If the first item is partially visible
        let to = if items.index(0).hidden(false) {
            // If the last item is completely hidden
            if items.index(items.len() - 1).hidden(true) {
                // Leave the scroll where it is
                scroll.yposition()
            // Otherwise,
            } else {
                // Move the scroll so that the last item is visible at the very bottom
                (items.len() as i32 - 1) * ITEM_HEIGHT - scroll.h() + 2
            }
        // Otherwise,
        } else {
            // Leave the scroll at the top
            0
        };

        // Remove the item from the Items and its frame from the Holder
        self.holder_mut().remove(items.index(index).frame());
        items.remove(index);

        // Resize the Holder (shrink it), change the scroll position
        // if hitting the bottom of the list, redraw the Scroll
        self.resize(items.len());
        scroll.scroll_to(0, to);
        scroll.redraw();
    }
}

impl Clone for Holder {
    fn clone(&self) -> Self {
        Holder {
            holder: Rc::clone(&self.holder),
        }
    }
}
