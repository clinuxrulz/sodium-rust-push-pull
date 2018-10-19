use sodium::impl_::Latch;
use sodium::impl_::MemoLazy;
use sodium::impl_::Node;
use sodium::impl_::SodiumCtx;
use sodium::impl_::Cell;
use sodium::impl_::gc::Finalize;
use sodium::impl_::gc::Gc;
use sodium::impl_::gc::Trace;
use std::cell::UnsafeCell;

pub struct CellLoop<A> {
    value: Gc<UnsafeCell<Latch<MemoLazy<A>>>>,
    node: Node
}

impl<A: Trace + Finalize + Clone + 'static> CellLoop<A> {
    pub fn new(sodium_ctx: &SodiumCtx) -> CellLoop<A> {
        unimplemented!();
    }

    pub fn loop_(&self, ca: Cell<A>) {
        unimplemented!();
    }

    pub fn to_cell(&self) -> Cell<A> {
        Cell {
            value: self.value.clone(),
            node: self.node.clone()
        }
    }
}

impl<A: Trace + Finalize + Clone + 'static> Clone for CellLoop<A> {
    fn clone(&self) -> Self {
        CellLoop {
            value: self.value.clone(),
            node: self.node.clone()
        }
    }
}