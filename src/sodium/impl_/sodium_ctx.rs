use sodium::gc::GcCtx;
use sodium::impl_::Node;
use std::cell::UnsafeCell;
use std::collections::BinaryHeap;
use std::mem::swap;
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
    pub transaction_depth: u32,
    pub to_be_updated: BinaryHeap<Node>,
    pub pre_trans: Vec<Box<FnMut()>>
}

impl SodiumCtx {
    pub fn new() -> SodiumCtx {
        SodiumCtx {
            data: Rc::new(UnsafeCell::new(SodiumCtxData {
                gc_ctx: GcCtx::new(),
                next_id: 0,
                transaction_depth: 0,
                to_be_updated: BinaryHeap::new(),
                pre_trans: Vec::new()
            }))
        }
    }

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

    pub fn pre_trans<F: FnMut() + 'static>(&self, f: F) {
        let self_ = unsafe { &mut *(*self.data).get() };
        self_.pre_trans.push(Box::new(f));
    }

    pub fn transaction<A,CODE:FnOnce()->A>(&self, code: CODE)->A {
        let self_ = unsafe { &mut *(*self.data).get() };
        self_.transaction_depth = self_.transaction_depth + 1;
        let result = code();
        self_.transaction_depth = self_.transaction_depth - 1;
        if self_.transaction_depth == 0 {
            self.propergate();
        }
        result
    }

    fn propergate(&self) {
        let self_ = unsafe { &mut *(*self.data).get() };
        {
            let mut pre_trans = Vec::new();
            swap(&mut self_.pre_trans, &mut pre_trans);
            for mut f in pre_trans {
                f();
            }
        }
        loop {
            let node_op = self_.to_be_updated.pop();
            match node_op {
                Some(node) => {
                    node.update();
                },
                None => break
            }
        }
    }
}

impl WeakSodiumCtx {
    pub fn upgrade(&self) -> Option<SodiumCtx> {
        self.data.upgrade().map(|data| SodiumCtx { data })
    }
}
