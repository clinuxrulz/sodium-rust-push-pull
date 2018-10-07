use sodium::gc::Finalize;
use sodium::gc::GcDep;
use sodium::gc::Trace;
use std::cell::UnsafeCell;
use std::rc::Rc;

pub struct MemoLazy<A> {
    thunk: Rc<Fn()->A>,
    val_op: UnsafeCell<Option<A>>
}

impl<A: Trace> Trace for MemoLazy<A> {
    fn trace(&self, f: &mut FnMut(&GcDep)) {
        self.get().trace(f);
    }
}

impl<A: Finalize> Finalize for MemoLazy<A> {
    fn finalize(&mut self) {
        self.get_mut().finalize();
    }
}

impl<A> MemoLazy<A> {
    pub fn new<F: Fn()->A + 'static>(thunk: F) -> MemoLazy<A> {
        MemoLazy {
            thunk: Rc::new(thunk),
            val_op: UnsafeCell::new(None)
        }
    }

    pub fn get(&self) -> &A {
        let val_op = unsafe { &*self.val_op.get() };
        match val_op {
            Some(ref val) => val,
            None => {
                let val = (self.thunk)();
                unsafe {
                    *self.val_op.get() = Some(val);
                }
                self.get()
            }
        }
    }

    pub fn get_mut(&mut self) -> &mut A {
        let val_op = unsafe { &mut *self.val_op.get() };
        match val_op {
            Some(ref mut val) => val,
            None => {
                let val = (self.thunk)();
                unsafe {
                    *self.val_op.get() = Some(val);
                }
                self.get_mut()
            }
        }
    }
}

impl<A: Clone> Clone for MemoLazy<A> {
    fn clone(&self) -> Self {
        let val_op = unsafe { &*self.val_op.get() };
        MemoLazy {
            thunk: self.thunk.clone(),
            val_op: UnsafeCell::new(val_op.clone())
        }
    }
}
