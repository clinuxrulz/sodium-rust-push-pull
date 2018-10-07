use sodium::IsLambda1;
use sodium::MemoLazy;
use sodium::Node;
use sodium::SodiumCtx;
use sodium::gc::Finalize;
use sodium::gc::Gc;
use sodium::gc::Trace;
use std::cell::UnsafeCell;
use std::rc::Rc;

pub struct Cell<A> {
    pub value: Gc<UnsafeCell<MemoLazy<A>>>,
    pub node: Node
}

impl<A: Clone + Trace + Finalize + 'static> Cell<A> {
    pub fn new(sodium_ctx: &SodiumCtx, value: A) -> Cell<A> {
        let mut gc_ctx = sodium_ctx.gc_ctx();
        Cell {
            value: gc_ctx.new_gc(UnsafeCell::new(MemoLazy::new(move || value.clone()))),
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
        thunk.get().clone()
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
        let self_2 = self_.clone();
        let rvalue;
        {
            let f = f.clone();
            rvalue = gc_ctx.new_gc(UnsafeCell::new(MemoLazy::new(move || f.apply(&self_.sample_no_trans()))));
        }
        let self_3 = self_2.clone();
        Cell {
            value: rvalue.clone(),
            node: Node::new(
                sodium_ctx,
                move || {
                    let thunk = unsafe { &mut *(*rvalue).get() };
                    let self_3 = self_3.clone();
                    let f = f.clone();
                    *thunk = MemoLazy::new(move || f.apply(&self_3.sample_no_trans()));
                },
                update_deps,
                vec![self_2.node.clone()],
                || {}
            )
        }
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