use std::cell::UnsafeCell;

pub struct MemoLazy<A> {
    thunk: Box<Fn()->A>,
    val_op: UnsafeCell<Option<A>>
}

impl<A> MemoLazy<A> {
    pub fn new<F: Fn()->A + 'static>(thunk: F) -> MemoLazy<A> {
        MemoLazy {
            thunk: Box::new(thunk),
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
}
