use std::rc::Rc;
use std::ops::Deref;
use std::cell::{RefCell, RefMut};

#[derive(Clone)]
pub struct MuttCounter {
    id: Rc<RefCell<i32>>,
}

impl MuttCounter {
    pub fn new() -> MuttCounter {
        MuttCounter {
            id: Rc::new(RefCell::new(0)),
        }
    }

    pub fn nextId(&mut self) -> i32 {
        // self.id.replace_with(|&mut old| old + 1);
        *self.id.borrow_mut() += 1;
        *self.id.borrow()
    }
}
