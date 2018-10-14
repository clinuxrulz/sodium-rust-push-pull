use sodium::impl_::Node;
use sodium::gc::Finalize;
use sodium::gc::Gc;
use sodium::gc::GcDep;
use sodium::gc::Trace;
use std::cell::UnsafeCell;

pub struct Listener {
    node_op: Gc<UnsafeCell<Option<Node>>>
}

impl Listener {
    pub fn new(node: Node) -> Listener {
        let sodium_ctx = node.sodium_ctx();
        let mut gc_ctx = sodium_ctx.gc_ctx();
        Listener {
            node_op: gc_ctx.new_gc(UnsafeCell::new(Some(node)))
        }
    }

    pub fn unlisten(&self) {
        let node_op = unsafe { &mut *(*self.node_op).get() };
        *node_op = None;
    }
}
