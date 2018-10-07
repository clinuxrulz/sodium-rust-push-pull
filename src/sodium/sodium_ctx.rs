use sodium::gc::GcCtx;
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
    pub gc_ctx: GcCtx,
    pub next_id: u32,
    pub to_be_updated: BinaryHeap<Node>
}

impl SodiumCtx {
    pub fn gc_ctx(&self) -> GcCtx {
        let self_ = unsafe { &*(*self.data).get() };
        self_.gc_ctx.clone()
    }

    pub fn downgrade(&self) -> WeakSodiumCtx {
        WeakSodiumCtx {
            data: Rc::downgrade(&self.data)
        }
    }

    pub fn new_id(&self) -> u32 {
        let self_ = unsafe { &mut *(*self.data).get() };
        let id = self_.next_id;
        self_.next_id = self_.next_id + 1;
        id
    }
}

impl WeakSodiumCtx {
    pub fn upgrade(&self) -> Option<SodiumCtx> {
        self.data.upgrade().map(|data| SodiumCtx { data })
    }
}
