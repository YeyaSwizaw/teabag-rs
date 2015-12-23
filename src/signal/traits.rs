use std::rc::Rc;
use std::cell::RefCell;

pub trait Slot<F> {
    fn connect(&mut self, f: F);
}

pub trait Signal<T, A: Copy> {
    fn fire(&self, state: Rc<RefCell<T>>, args: A);
}
