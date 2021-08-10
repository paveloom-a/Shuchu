//! List's Selected is a wrapper around an index of the selected item.
//!
//! Note that the counting of the items starts from 1.

use std::{cell::RefCell, rc::Rc};

/// A wrapper around an index of the selected item
pub struct Selected {
    /// A reference-counting pointer to an index of the selected item
    selected: Rc<RefCell<usize>>,
}

impl Selected {
    /// Get the default struct
    pub fn default() -> Self {
        Selected {
            selected: Rc::<RefCell<usize>>::default(),
        }
    }

    /// Get the index
    pub fn get(&self) -> usize {
        *self.selected.borrow()
    }

    /// Get the index into a vector
    pub fn index(&self) -> usize {
        self.get() - 1
    }

    /// Decrement the index
    pub fn decrement(&self) {
        self.set(self.get() - 1);
    }

    /// Increment the index
    pub fn increment(&self) {
        self.set(self.get() + 1);
    }

    /// Set the index to the passed value
    pub fn set(&self, idx: usize) {
        *self.selected.borrow_mut() = idx;
    }
}

impl Clone for Selected {
    fn clone(&self) -> Self {
        Selected {
            selected: Rc::clone(&self.selected),
        }
    }
}
