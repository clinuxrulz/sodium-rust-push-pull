use sodium::impl_::IsLambda1;
use sodium::impl_::IsLambda2;
use sodium::impl_::Latch;
use sodium::impl_::Listener;
use sodium::impl_::MemoLazy;
use sodium::impl_::Node;
use sodium::impl_::SodiumCtx;
use sodium::gc::Finalize;
use sodium::gc::Gc;
use sodium::gc::Trace;
use std::cell::UnsafeCell;
use std::rc::Rc;

pub struct Cell<A> {
    pub value: Gc<UnsafeCell<Latch<A>>>,
    pub node: Node
}

impl<A: Clone + Trace + Finalize + 'static> Cell<A> {
    pub fn new(sodium_ctx: &SodiumCtx, value: A) -> Cell<A> {
        let mut gc_ctx = sodium_ctx.gc_ctx();
        Cell {
            value: gc_ctx.new_gc(UnsafeCell::new(Latch::const_(MemoLazy::new(move || value.clone())))),
            node: Node::new(
                sodium_ctx,
                || {},
                Vec::new(),
                Vec::new(),
                || {}
            )
        }
    }

    fn sample_no_trans(&self) -> A {
        let thunk = unsafe { &*(*self.value).get() };
        thunk.get().get().clone()
    }

    pub fn map<B: Clone + Trace + Finalize + 'static,F:IsLambda1<A,B> + 'static>(
        &self,
        f: F
    ) -> Cell<B> {
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
                    f.apply(&self_.sample_no_trans())
                })
            }
        )));
        Cell {
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

    pub fn lift2<B,C,F: IsLambda2<A,B,C> + 'static>(&self, cb: Cell<B>, f: F) -> Cell<C> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static {
        let sodium_ctx = self.node.sodium_ctx();
        let sodium_ctx = &sodium_ctx;
        let mut gc_ctx = sodium_ctx.gc_ctx();
        let update_deps = f.deps();
        let f = Rc::new(f);
        let self_ = self.clone();
        let self_2 = self.clone();
        let latch;
        {
            let cb = cb.clone();
            latch = gc_ctx.new_gc(UnsafeCell::new(Latch::new(
                move || {
                    let self_ = self_.clone();
                    let cb = cb.clone();
                    let f = f.clone();
                    MemoLazy::new(move || {
                        f.apply(&self_.sample_no_trans(), &cb.sample_no_trans())
                    })
                }
            )));
        }
        Cell {
            value: latch.clone(),
            node: Node::new(
                sodium_ctx,
                move || {
                    let latch = unsafe { &mut *(*latch).get() };
                    latch.reset();
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
            sodium_ctx.pre(move || {
                let callback = unsafe { &mut *(*callback).get() };
                (*callback)(&self_.sample_no_trans());
            });
        }
        Listener::new(Node::new(
            sodium_ctx,
            move || {
                let callback = unsafe { &mut *(*callback).get() };
                (*callback)(&self_.sample_no_trans());
            },
            Vec::new(),
            vec![self.node.clone()],
            || {}
        ))
    }
}

impl<A: Clone + 'static> Clone for Cell<A> {
    fn clone(&self) -> Self {
        Cell {
            value: self.value.clone(),
            node: self.node.clone()
        }
    }
}