use sodium::gc::Finalize;
use sodium::gc::GcDep;
use sodium::gc::Trace;
use sodium::impl_::MemoLazy;
use std::rc::Rc;

pub struct Latch<A> {
    thunk: Rc<Fn()->MemoLazy<A>>,
    val: MemoLazy<A>
}

impl<A: Trace> Trace for Latch<A> {
    fn trace(&self, f: &mut FnMut(&GcDep)) {
        self.val.trace(f);
    }
}

impl<A: Finalize> Finalize for Latch<A> {
    fn finalize(&mut self) {
        self.val.finalize();
    }
}

impl<A:Clone + 'static> Latch<A> {
    pub fn const_(value:  MemoLazy<A>) -> Latch<A> {
        Latch::new(move || value.clone())
    }
}

impl<A> Latch<A> {
    pub fn new<F: Fn()->MemoLazy<A> + 'static>(thunk: F) -> Latch<A> {
        let val = thunk();
        Latch {
            thunk: Rc::new(thunk),
            val
        }
    }

    pub fn get(&self) -> &MemoLazy<A> {
        &self.val
    }

    pub fn get_mut(&mut self) -> &mut MemoLazy<A> {
        &mut self.val
    }

    pub fn reset(&mut self) {
        self.val = (self.thunk)();
    }
}

impl<A: Clone> Clone for Latch<A> {
    fn clone(&self) -> Self {
        Latch {
            thunk: self.thunk.clone(),
            val: self.val.clone()
        }
    }
}
