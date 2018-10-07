use sodium::gc::GcWeak;
use sodium::Node;
use std::cell::UnsafeCell;
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::rc::Weak;

pub struct SodiumCtx {
    pub data: Rc<UnsafeCell<SodiumCtxData>>
}

pub struct WeakSodiumCtx {
    pub data: Weak<UnsafeCell<SodiumCtxData>>
}

pub struct SodiumCtxData {
    pub to_be_updated: BinaryHeap<Node>
}

impl SodiumCtx {
    pub fn downgrade(&self) -> WeakSodiumCtx {
        WeakSodiumCtx {
            data: Rc::downgrade(&self.data)
        }
    }
}

impl WeakSodiumCtx {
    pub fn upgrade(&self) -> Option<SodiumCtx> {
        self.data.upgrade().map(|data| SodiumCtx { data })
    }
}
