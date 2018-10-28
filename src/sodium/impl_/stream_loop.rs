use sodium::impl_::Latch;
use sodium::impl_::MemoLazy;
use sodium::impl_::Node;
use sodium::impl_::SodiumCtx;
use sodium::impl_::Stream;
use sodium::impl_::gc::Finalize;
use sodium::impl_::gc::Gc;
use sodium::impl_::gc::Trace;
use std::cell::UnsafeCell;

pub struct StreamLoop<A> {
    value: Gc<UnsafeCell<Option<MemoLazy<A>>>>,
    node: Node
}

impl<A: Trace + Finalize + Clone + 'static> StreamLoop<A> {
    pub fn new(sodium_ctx: &SodiumCtx) -> StreamLoop<A> {
        unimplemented!();
    }

    pub fn loop_(&self, sa: Stream<A>) {
        unimplemented!();
    }

    pub fn to_stream(&self) -> Stream<A> {
        Stream {
            value: self.value.clone(),
            node: self.node.clone()
        }
    }
}

impl<A: Trace + Finalize + Clone + 'static> Clone for StreamLoop<A> {
    fn clone(&self) -> Self {
        StreamLoop {
            value: self.value.clone(),
            node: self.node.clone()
        }
    }
}