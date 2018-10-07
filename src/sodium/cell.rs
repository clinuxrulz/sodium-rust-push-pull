use sodium::gc::Finalize;
use sodium::gc::Gc;
use sodium::gc::Trace;
use sodium::MemoLazy;
use sodium::Node;
use sodium::SodiumCtx;
use std::cell::UnsafeCell;

pub struct Cell<A> {
    value: Gc<UnsafeCell<MemoLazy<A>>>,
    node: Node
}

impl<A: Clone + Trace + Finalize + 'static> Cell<A> {
    pub fn new(sodium_ctx: &SodiumCtx, value: A) -> Cell<A> {
        let mut gc_ctx = sodium_ctx.gc_ctx();
        Cell {
            value: gc_ctx.new_gc(UnsafeCell::new(MemoLazy::new(move || value.clone()))),
            node: Node::new(
                sodium_ctx,
                || {},
                Vec::new(),
                || {}
            )
        }
    }
}
