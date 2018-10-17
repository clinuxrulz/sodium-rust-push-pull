use sodium::impl_::Cell;
use sodium::impl_::Dep;
use sodium::impl_::IsLambda1;
use sodium::impl_::IsLambda2;
use sodium::impl_::IsLambda3;
use sodium::impl_::IsLambda4;
use sodium::impl_::IsLambda5;
use sodium::impl_::IsLambda6;
use sodium::impl_::Lambda;
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

pub struct Stream<A> {
    pub value: Gc<UnsafeCell<Latch<Option<MemoLazy<A>>>>>,
    pub node: Node
}

impl<A: Clone + Trace + Finalize + 'static> Stream<Option<A>> {
    pub fn filter_option(&self) -> Stream<A> {
        let sodium_ctx = self.node.sodium_ctx();
        let sodium_ctx = &sodium_ctx;
        let mut gc_ctx = sodium_ctx.gc_ctx();
        let self_ = self.clone();
        let self_2 = self.clone();
        let latch = gc_ctx.new_gc(UnsafeCell::new(Latch::new(
            move || {
                let self_ = self_.clone();
                let val_op_op = self_.peek_value();
                match val_op_op {
                    Some(val_op) => {
                        val_op.get().clone().map(|val| {
                            MemoLazy::new(move || val.clone())
                        })
                    },
                    None => None
                }
            }
        )));
        Stream {
            value: latch.clone(),
            node: Node::new(
                sodium_ctx,
                move || {
                    let latch = unsafe { &mut *(*latch).get() };
                    latch.reset();
                    return true;
                },
                Vec::new(),
                vec![self_2.node.clone()],
                || {}
            )
        }
    }
}

impl<A: Clone + Trace + Finalize + 'static> Stream<A> {
    pub fn new(sodium_ctx: &SodiumCtx) -> Stream<A> {
        let mut gc_ctx = sodium_ctx.gc_ctx();
        Stream {
            value: gc_ctx.new_gc(UnsafeCell::new(Latch::const_(None))),
            node: Node::new(
                sodium_ctx,
                || false,
                Vec::new(),
                Vec::new(),
                || {}
            )
        }
    }

    pub fn to_dep(&self) -> Dep {
        self.node.to_dep()
    }

    fn peek_value(&self) -> Option<MemoLazy<A>> {
        let thunk = unsafe { &*(*self.value).get() };
        thunk.get().clone()
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
                self_.peek_value().map(|val| {
                    MemoLazy::new(move || f.apply(val.get()))
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
                    return true;
                },
                update_deps,
                vec![self_2.node.clone()],
                || {}
            )
        }
    }

    pub fn hold(&self, a: A) -> Cell<A> {
        let sodium_ctx = self.node.sodium_ctx();
        let sodium_ctx = &sodium_ctx;
        let mut gc_ctx = sodium_ctx.gc_ctx();
        let val: Rc<UnsafeCell<MemoLazy<A>>>;
        val = Rc::new(UnsafeCell::new(
            match self.peek_value() {
                Some(val) => val,
                None => MemoLazy::new(move || a.clone())
            }
        ));
        let latch;
        {
            let self_ = self.clone();
            latch = gc_ctx.new_gc(UnsafeCell::new(Latch::new(move || {
                let val: &mut MemoLazy<A> = unsafe { &mut *(*val).get() };
                match self_.peek_value() {
                    Some(val2) => *val = val2,
                    None => ()
                }
                val.clone()
            })));
        }
        let self_ = self.clone();
        Cell {
            value: latch.clone(),
            node: Node::new(
                sodium_ctx,
                move || {
                    let latch = unsafe { &mut *(*latch).get() };
                    latch.reset();
                    return self_.peek_value().is_some();
                },
                Vec::new(),
                vec![self.node.clone()],
                || {}
            )
        }
    }

    pub fn filter<PRED:IsLambda1<A,bool> + 'static>(&self, pred: PRED) -> Stream<A> {
        let sodium_ctx = self.node.sodium_ctx();
        let sodium_ctx = &sodium_ctx;
        let mut gc_ctx = sodium_ctx.gc_ctx();
        let update_deps = pred.deps();
        let pred = Rc::new(pred);
        let self_ = self.clone();
        let self_2 = self.clone();
        let latch = gc_ctx.new_gc(UnsafeCell::new(Latch::new(
            move || {
                let self_ = self_.clone();
                let pred = pred.clone();
                match self_.peek_value() {
                    Some(val) => {
                        let val = val.get();
                        if pred.apply(val) {
                            let val = val.clone();
                            Some(MemoLazy::new(move || val.clone()))
                        } else {
                            None
                        }
                    },
                    None => None
                }
            }
        )));
        Stream {
            value: latch.clone(),
            node: Node::new(
                sodium_ctx,
                move || {
                    let latch = unsafe { &mut *(*latch).get() };
                    latch.reset();
                    return true;
                },
                update_deps,
                vec![self_2.node.clone()],
                || {}
            )
        }
    }

    pub fn snapshot<B>(&self, cb: Cell<B>) -> Stream<B> where B: Trace + Finalize + Clone + 'static {
        let deps = vec![cb.to_dep()];
        self.map(Lambda::new(Box::new(move |_a: &A| cb.sample_no_trans()), deps))
    }

    pub fn snapshot2<B,C,FN:IsLambda2<A,B,C> + 'static>(&self, cb: Cell<B>, f: FN) -> Stream<C> where B: Trace + Finalize + Clone + 'static, C: Trace + Finalize + Clone + 'static {
        let mut deps = f.deps();
        deps.push(cb.to_dep());
        self.map(Lambda::new(Box::new(move |a: &A| f.apply(a, &cb.sample_no_trans())), deps))
    }

    pub fn snapshot3<B,C,D,FN:IsLambda3<A,B,C,D> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, f: FN) -> Stream<D> where B: Trace + Finalize + Clone + 'static, C: Trace + Finalize + Clone + 'static, D: Trace + Finalize + Clone + 'static {
        let mut deps = f.deps();
        deps.push(cb.to_dep());
        deps.push(cc.to_dep());
        self.map(Lambda::new(Box::new(move |a: &A| f.apply(a, &cb.sample_no_trans(), &cc.sample_no_trans())), deps))
    }

    pub fn snapshot4<B,C,D,E,FN:IsLambda4<A,B,C,D,E> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, f: FN) -> Stream<E> where B: Trace + Finalize + Clone + 'static, C: Trace + Finalize + Clone + 'static, D: Trace + Finalize + Clone + 'static, E: Trace + Finalize + Clone + 'static {
        let mut deps = f.deps();
        deps.push(cb.to_dep());
        deps.push(cc.to_dep());
        deps.push(cd.to_dep());
        self.map(Lambda::new(Box::new(move |a: &A| f.apply(a, &cb.sample_no_trans(), &cc.sample_no_trans(), &cd.sample_no_trans())), deps))
    }

    pub fn snapshot5<B,C,D,E,F,FN:IsLambda5<A,B,C,D,E,F> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, ce: Cell<E>, f: FN) -> Stream<F> where B: Trace + Finalize + Clone + 'static, C: Trace + Finalize + Clone + 'static, D: Trace + Finalize + Clone + 'static, E: Trace + Finalize + Clone + 'static, E: Trace + Finalize + Clone + 'static, F: Trace + Finalize + Clone + 'static {
        let mut deps = f.deps();
        deps.push(cb.to_dep());
        deps.push(cc.to_dep());
        deps.push(cd.to_dep());
        deps.push(ce.to_dep());
        self.map(Lambda::new(Box::new(move |a: &A| f.apply(a, &cb.sample_no_trans(), &cc.sample_no_trans(), &cd.sample_no_trans(), &ce.sample_no_trans())), deps))
    }

    pub fn snapshot6<B,C,D,E,F,G,FN:IsLambda6<A,B,C,D,E,F,G> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, ce: Cell<E>, cf: Cell<F>, f: FN) -> Stream<G> where B: Trace + Finalize + Clone + 'static, C: Trace + Finalize + Clone + 'static, D: Trace + Finalize + Clone + 'static, E: Trace + Finalize + Clone + 'static, E: Trace + Finalize + Clone + 'static, F: Trace + Finalize + Clone + 'static, G: Trace + Finalize + Clone + 'static {
        let mut deps = f.deps();
        deps.push(cb.to_dep());
        deps.push(cc.to_dep());
        deps.push(cd.to_dep());
        deps.push(ce.to_dep());
        deps.push(cf.to_dep());
        self.map(Lambda::new(Box::new(move |a: &A| f.apply(a, &cb.sample_no_trans(), &cc.sample_no_trans(), &cd.sample_no_trans(), &ce.sample_no_trans(), &cf.sample_no_trans())), deps))
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
                sodium_ctx.pre(move || {
                    let callback = unsafe { &mut *(*callback).get() };
                    (*callback)(value.get());
                });
            }
        }
        Listener::new(Node::new(
            sodium_ctx,
            move || {
                let callback = unsafe { &mut *(*callback).get() };
                let value_op = self_.peek_value();
                if let Some(value) = value_op {
                    (*callback)(value.get());
                }
                return false;
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
