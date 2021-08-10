//! List's Items is a wrapper around a vector of the [`Item`](super::Item)s.

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use super::item::Item;

/// A wrapper around a vector of the [`Item`](super::Item)s
pub struct Items {
    /// A reference-counting pointer to a mutable vector of the [`Item`](super::Item)s
    items: Rc<RefCell<Vec<Item>>>,
}

impl Items {
    /// Get the default struct
    pub fn default() -> Self {
        Items {
            items: Rc::<RefCell<Vec<Item>>>::default(),
        }
    }

    /// Get an immutable reference to the vector
    fn items(&self) -> Ref<Vec<Item>> {
        self.items.borrow()
    }

    /// Get a mutable reference to the vector
    fn items_mut(&self) -> RefMut<Vec<Item>> {
        self.items.borrow_mut()
    }

    /// Get the length of the vector
    pub fn len(&self) -> usize {
        self.items().len()
    }

    /// Get an immutable reference to an item at the specified index
    pub fn index(&self, index: usize) -> Ref<Item> {
        Ref::map(self.items(), |items| &items[index])
    }

    /// Get a mutable reference to an item at the specified index
    pub fn index_mut(&self, index: usize) -> RefMut<Item> {
        RefMut::map(self.items_mut(), |items| &mut items[index])
    }

    /// Push an item to the vector
    pub fn push(&self, item: Item) {
        self.items_mut().push(item);
    }

    /// Select an item at the specified index, unselecting all others.
    ///
    /// Counting of the items here starts from 1. Selecting 0, or
    /// any other value outside the bounds will unselect all items.
    pub fn select(&self, idx: usize) {
        // If the index is in the limits of the vector (if counting from 1)
        if idx > 0 && idx <= self.len() {
            // Get the actual index
            let idx = idx - 1;
            // Select the specified item, unselecting all others
            for i in 0..self.len() {
                if i == idx {
                    self.index_mut(i).select();
                } else if self.index(i).selected() {
                    self.index_mut(i).unselect();
                }
            }
        // Otherwise,
        } else {
            // Unselect all items
            for i in 0..self.len() {
                self.index_mut(i).unselect();
            }
        }
    }

    /// Remove an item at the specified index from the vector
    pub fn remove(&self, index: usize) {
        self.items_mut().remove(index);
    }
}

impl Clone for Items {
    fn clone(&self) -> Self {
        Items {
            items: Rc::clone(&self.items),
        }
    }
}
