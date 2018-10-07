use sodium::gc::Gc;
use sodium::MemoLazy;
use sodium::Node;
use std::cell::UnsafeCell;

pub struct Cell<A> {
    value: Gc<UnsafeCell<MemoLazy<A>>>,
    node: Node
}
