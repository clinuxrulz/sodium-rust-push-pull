use sodium::gc::Gc;
use sodium::MemoLazy;
use sodium::Node;
use sodium::SodiumCtx;
use std::cell::UnsafeCell;

pub struct Cell<A> {
    value: Gc<UnsafeCell<MemoLazy<A>>>,
    node: Node
}

impl<A> Cell<A> {
    pub fn new(sodium_ctx: &SodiumCtx, value: A) {
        
    }
}