use sodium::MemoLazy;
use std::rc::Rc;

pub struct Latch<A> {
    thunk: Rc<Fn()->MemoLazy<A>>,
    val: MemoLazy<A>
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
