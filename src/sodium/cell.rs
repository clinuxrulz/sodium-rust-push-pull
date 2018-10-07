use sodium::gc::Gc;
use sodium::Node;
use std::cell::UnsafeCell;

pub struct Cell<A> {
    value: Gc<UnsafeCell<A>>,
    node: Gc<UnsafeCell<Node>>
}
