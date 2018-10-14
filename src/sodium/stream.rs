use sodium::IsLambda1;
use sodium::Latch;
use sodium::Listener;
use sodium::MemoLazy;
use sodium::Node;
use sodium::SodiumCtx;
use sodium::gc::Finalize;
use sodium::gc::Gc;
use sodium::gc::Trace;
use std::cell::UnsafeCell;
use std::rc::Rc;

pub struct Stream<A> {
    pub value: Gc<UnsafeCell<Latch<Option<A>>>>,
    pub node: Node
}

impl<A: Clone + Trace + Finalize + 'static> Stream<A> {
    pub fn new(sodium_ctx: &SodiumCtx) -> Stream<A> {
        let mut gc_ctx = sodium_ctx.gc_ctx();
        Stream {
            value: gc_ctx.new_gc(UnsafeCell::new(Latch::const_(MemoLazy::new(move || None)))),
            node: Node::new(
                sodium_ctx,
                || {},
                Vec::new(),
                Vec::new(),
                || {}
            )
        }
    }

    fn peek_value(&self) -> Option<A> {
        let thunk = unsafe { &*(*self.value).get() };
        thunk.get().get().clone()
    }

    pub fn map<B: Clone + Trace + Finalize + 'static,F:IsLambda1<A,B> + 'static>(
        &self,
        f: F
    ) -> Stream<B> {
        let sodium_ctx = self.node.sodium_ctx();
        let sodium_ctx = &sodium_ctx;
        let mut gc_ctx = sodium_ctx.gc_ctx();
        let update_deps = f.deps();
        let f = Rc::new(f);
        let self_ = self.clone();
        let self_2 = self.clone();
        let rval_latch = gc_ctx.new_gc(UnsafeCell::new(Latch::new(
            move || {
                let self_ = self_.clone();
                let f = f.clone();
                MemoLazy::new(move || {
                    self_.peek_value().map(|value| f.apply(&value))
                })
            }
        )));
        Stream {
            value: rval_latch.clone(),
            node: Node::new(
                sodium_ctx,
                move || {
                    let rval_latch = unsafe { &mut *(*rval_latch).get() };
                    rval_latch.reset();
                },
                update_deps,
                vec![self_2.node.clone()],
                || {}
            )
        }
    }

    pub fn listen<CALLBACK:FnMut(&A)+'static>(
        &self,
        callback: CALLBACK
    ) -> Listener {
        let sodium_ctx = self.node.sodium_ctx();
        let sodium_ctx = &sodium_ctx;
        let callback = Rc::new(UnsafeCell::new(callback));
        let self_ = self.clone();
        {
            let self_ = self_.clone();
            let callback = callback.clone();
            let value_op = self_.peek_value();
            if let Some(value) = value_op {
                sodium_ctx.pre_trans(move || {
                    let callback = unsafe { &mut *(*callback).get() };
                    (*callback)(&value);
                });
            }
        }
        Listener::new(Node::new(
            sodium_ctx,
            move || {
                let callback = unsafe { &mut *(*callback).get() };
                let value_op = self_.peek_value();
                if let Some(value) = value_op {
                    (*callback)(&value);
                }
            },
            Vec::new(),
            vec![self.node.clone()],
            || {}
        ))
    }
}

impl<A:Clone + 'static> Clone for Stream<A> {
    fn clone(&self) -> Self {
        Stream {
            value: self.value.clone(),
            node: self.node.clone()
        }
    }
}
