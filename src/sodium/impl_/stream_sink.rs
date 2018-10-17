use sodium::impl_::Stream;
use sodium::impl_::Latch;
use sodium::impl_::MemoLazy;
use sodium::impl_::Node;
use sodium::impl_::SodiumCtx;
use sodium::gc::Finalize;
use sodium::gc::Gc;
use sodium::gc::Trace;
use std::cell::UnsafeCell;
use std::mem::swap;
use std::rc::Rc;

pub struct StreamSink<A> {
    value: Gc<UnsafeCell<Latch<MemoLazy<Option<A>>>>>,
    next_value_op: Gc<UnsafeCell<Option<Latch<MemoLazy<Option<A>>>>>>,
    node: Node,
    will_clear: Rc<UnsafeCell<bool>>
}

impl<A: Trace + Finalize + Clone + 'static> StreamSink<A> {
    pub fn new(sodium_ctx: &SodiumCtx) -> StreamSink<A> {
        let mut gc_ctx = sodium_ctx.gc_ctx();
        let value = gc_ctx.new_gc(UnsafeCell::new(Latch::const_(MemoLazy::new(|| None))));
        let next_value_op = gc_ctx.new_gc(UnsafeCell::new(None));
        StreamSink {
            value: value.clone(),
            next_value_op: next_value_op.clone(),
            node: Node::new(
                sodium_ctx,
                move || {
                    let next_value_op = unsafe { &mut *(*next_value_op).get() };
                    let mut next_value_op2 = None;
                    swap(next_value_op, &mut next_value_op2);
                    let value = unsafe { &mut *(*value).get() };
                    if let Some(next_value) = next_value_op {
                        *value = next_value.clone();
                    }
                    return true;
                },
                Vec::new(),
                Vec::new(),
                || {}
            ),
            will_clear: Rc::new(UnsafeCell::new(false))
        }
    }

    pub fn send(&self, value: A) {
        let sodium_ctx = self.node.sodium_ctx();
        let sodium_ctx2 = sodium_ctx.clone();
        sodium_ctx.transaction(|| {
            let will_clear = unsafe { &mut *(*self.will_clear).get() };
            if !*will_clear {
                *will_clear = true;
                let self_ = self.clone();
                sodium_ctx.post(move || {
                    let value = unsafe { &mut *(*self_.value).get() };
                    let will_clear = unsafe { &mut *(*self_.will_clear).get() };
                    *value = Latch::const_(MemoLazy::new(|| None));
                    *will_clear = false;
                });
            }
            let next_value_op = unsafe { &mut *(*self.next_value_op).get() };
            *next_value_op = Some(Latch::const_(MemoLazy::new(move || Some(value.clone()))));
            self.node.mark_dirty();
        });
    }

    pub fn to_stream(&self) -> Stream<A> {
        Stream {
            value: self.value.clone(),
            node: self.node.clone()
        }
    }
}

impl<A: Clone + Trace + Finalize + 'static> Clone for StreamSink<A> {
    fn clone(&self) -> Self {
        StreamSink {
            value: self.value.clone(),
            next_value_op: self.next_value_op.clone(),
            node: self.node.clone(),
            will_clear: self.will_clear.clone()
        }
    }
}
