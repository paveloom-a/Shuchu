use std::{cell::RefCell, rc::Rc};

type Index = usize;
type IndexRc = Rc<RefCell<Index>>;

pub struct Selected {
    selected: IndexRc,
}

impl Selected {
    pub fn default() -> Self {
        Selected {
            selected: IndexRc::default(),
        }
    }

    pub fn get(&self) -> Index {
        *self.selected.borrow()
    }

    pub fn index(&self) -> Index {
        self.get() - 1
    }

    pub fn decrement(&mut self) {
        self.set(self.get() - 1)
    }

    pub fn increment(&mut self) {
        self.set(self.get() + 1)
    }

    pub fn set(&self, idx: Index) {
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
