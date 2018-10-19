use sodium::impl_::Cell;
use sodium::impl_::Latch;
use sodium::impl_::MemoLazy;
use sodium::impl_::Node;
use sodium::impl_::SodiumCtx;
use sodium::gc::Finalize;
use sodium::gc::Gc;
use sodium::gc::Trace;
use std::cell::UnsafeCell;
use std::mem::swap;

pub struct CellSink<A> {
    value: Gc<UnsafeCell<Latch<MemoLazy<A>>>>,
    next_value_op: Gc<UnsafeCell<Option<Latch<MemoLazy<A>>>>>,
    node: Node
}

impl<A: Trace + Finalize + Clone + 'static> CellSink<A> {
    pub fn new(sodium_ctx: &SodiumCtx, value: A) -> CellSink<A> {
        let mut gc_ctx = sodium_ctx.gc_ctx();
        let value = gc_ctx.new_gc(UnsafeCell::new(Latch::const_(MemoLazy::new(move || value.clone()))));
        let next_value_op = gc_ctx.new_gc(UnsafeCell::new(None));
        CellSink {
            value: value.clone(),
            next_value_op: next_value_op.clone(),
            node: Node::new(
                sodium_ctx,
                move || {
                    let next_value_op = unsafe { &mut *(*next_value_op).get() };
                    let mut next_value_op2 = None;
                    swap(next_value_op, &mut next_value_op2);
                    match next_value_op2 {
                        Some(next_value) => {
                            let value = unsafe { &mut *(*value).get() };
                            *value = next_value;
                        },
                        None => ()
                    }
                    return true;
                },
                Vec::new(),
                Vec::new(),
                || {}
            )
        }
    }

    pub fn send(&self, value: A) {
        let sodium_ctx = self.node.sodium_ctx();
        sodium_ctx.transaction(|| {
            let next_value_op = unsafe { &mut *(*self.next_value_op).get() };
            *next_value_op = Some(Latch::const_(MemoLazy::new(move || value.clone())));
            self.node.mark_dirty();
        });
    }

    pub fn to_cell(&self) -> Cell<A> {
        Cell {
            value: self.value.clone(),
            node: self.node.clone()
        }
    }
}

impl<A: Trace + Finalize + Clone + 'static> Clone for CellSink<A> {
    fn clone(&self) -> Self {
        CellSink {
            value: self.value.clone(),
            next_value_op: self.next_value_op.clone(),
            node: self.node.clone()
        }
    }
}
