use ::signal::{Signal, Slot};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Signal0<T> {
    fs: Vec<Box<Fn(&mut T)>>
}

pub struct Signal1<T, A> {
    fs: Vec<Box<Fn(&mut T, A)>>
}

impl<T, F: 'static + Fn(&mut T)> Slot<F> for Signal0<T> {
    fn connect(&mut self, f: F) {
        self.fs.push(Box::new(f));
    }
}

impl<T, A: Copy, F: 'static + Fn(&mut T, A)> Slot<F> for Signal1<T, A> {
    fn connect(&mut self, f: F) {
        self.fs.push(Box::new(f));
    }
}

impl<T> Signal<T, ()> for Signal0<T> {
    fn fire(&self, state: Rc<RefCell<T>>, _: ()) {
        for f in self.fs.iter() {
            f(&mut state.borrow_mut());
        }
    }
}

impl<T, A: Copy> Signal<T, (A,)> for Signal1<T, A> {
    fn fire(&self, state: Rc<RefCell<T>>, (arg,): (A,)) {
        for f in self.fs.iter() {
            f(&mut state.borrow_mut(), arg);
        }
    }
}

impl<T> Default for Signal0<T> {
    fn default() -> Signal0<T> {
        Signal0 {
            fs: Vec::new()
        }
    }
}

impl<T, A> Default for Signal1<T, A> {
    fn default() -> Signal1<T, A> {
        Signal1 {
            fs: Vec::new()
        }
    }
}
