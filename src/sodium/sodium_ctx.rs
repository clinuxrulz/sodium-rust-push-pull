use sodium::gc::GcWeak;
use sodium::Node;
use std::cell::UnsafeCell;
use std::collections::BinaryHeap;
use std::rc::Rc;

pub struct SodiumCtx {
    data: Rc<SodiumCtxData>
}

struct SodiumCtxData {
    to_be_updated: BinaryHeap<GcWeak<UnsafeCell<Node>>>
}