use sodium::impl_::Dep;
use sodium::impl_::IsLambda1;
use sodium::impl_::IsLambda2;
use sodium::impl_::IsLambda3;
use sodium::impl_::IsLambda4;
use sodium::impl_::IsLambda5;
use sodium::impl_::IsLambda6;
use sodium::impl_::Latch;
use sodium::impl_::Listener;
use sodium::impl_::MemoLazy;
use sodium::impl_::Node;
use sodium::impl_::SodiumCtx;
use sodium::impl_::Stream;
use sodium::gc::Finalize;
use sodium::gc::Gc;
use sodium::gc::GcDep;
use sodium::gc::Trace;
use std::cell::UnsafeCell;
use std::rc::Rc;

pub struct Cell<A> {
    pub value: Gc<UnsafeCell<Latch<MemoLazy<A>>>>,
    pub node: Node
}

impl<A: Clone + Trace + Finalize + 'static> Cell<A> {
    pub fn new(sodium_ctx: &SodiumCtx, value: A) -> Cell<A> {
        let mut gc_ctx = sodium_ctx.gc_ctx();
        Cell {
            value: gc_ctx.new_gc(UnsafeCell::new(Latch::const_(MemoLazy::new(move || value.clone())))),
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

    pub fn sample_no_trans(&self) -> A {
        let thunk = unsafe { &*(*self.value).get() };
        thunk.get().get().clone()
    }

    pub fn sample(&self) -> A {
        let sodium_ctx = self.node.sodium_ctx();
        let sodium_ctx = &sodium_ctx;
        sodium_ctx.transaction(|| self.sample_no_trans())
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
                    return true;
                },
                update_deps,
                vec![self_2.node.clone()],
                || {}
            )
        }
    }

    pub fn apply<B,F: IsLambda1<A,B> + Trace + Finalize + Clone + 'static>(&self, cf: Cell<F>) -> Cell<B> where B: Trace + Finalize + Clone + 'static {
        self.lift2(cf, |a: &A, f: &F| f.apply(a))
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
                    return true;
                },
                update_deps,
                vec![self_2.node.clone(), cb.node.clone()],
                || {}
            )
        }
    }

    pub fn lift3<B,C,D,F: IsLambda3<A,B,C,D> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, f: F) -> Cell<D> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static {
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
            let cc = cc.clone();
            latch = gc_ctx.new_gc(UnsafeCell::new(Latch::new(
                move || {
                    let self_ = self_.clone();
                    let cb = cb.clone();
                    let cc = cc.clone();
                    let f = f.clone();
                    MemoLazy::new(move || {
                        f.apply(&self_.sample_no_trans(), &cb.sample_no_trans(), &cc.sample_no_trans())
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
                    return true;
                },
                update_deps,
                vec![self_2.node.clone(), cb.node.clone(), cc.node.clone()],
                || {}
            )
        }
    }

    pub fn lift4<B,C,D,E,F: IsLambda4<A,B,C,D,E> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, f: F) -> Cell<E> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static, E: Clone + Trace + Finalize + 'static {
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
            let cc = cc.clone();
            let cd = cd.clone();
            latch = gc_ctx.new_gc(UnsafeCell::new(Latch::new(
                move || {
                    let self_ = self_.clone();
                    let cb = cb.clone();
                    let cc = cc.clone();
                    let cd = cd.clone();
                    let f = f.clone();
                    MemoLazy::new(move || {
                        f.apply(&self_.sample_no_trans(), &cb.sample_no_trans(), &cc.sample_no_trans(), &cd.sample_no_trans())
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
                    return true;
                },
                update_deps,
                vec![self_2.node.clone(), cb.node.clone(), cc.node.clone(), cd.node.clone()],
                || {}
            )
        }
    }

    pub fn lift5<B,C,D,E,F,FN: IsLambda5<A,B,C,D,E,F> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, ce: Cell<E>, f: FN) -> Cell<F> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static, E: Clone + Trace + Finalize + 'static, F: Clone + Trace + Finalize + 'static {
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
            let cc = cc.clone();
            let cd = cd.clone();
            let ce = ce.clone();
            latch = gc_ctx.new_gc(UnsafeCell::new(Latch::new(
                move || {
                    let self_ = self_.clone();
                    let cb = cb.clone();
                    let cc = cc.clone();
                    let cd = cd.clone();
                    let ce = ce.clone();
                    let f = f.clone();
                    MemoLazy::new(move || {
                        f.apply(&self_.sample_no_trans(), &cb.sample_no_trans(), &cc.sample_no_trans(), &cd.sample_no_trans(), &ce.sample_no_trans())
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
                    return true;
                },
                update_deps,
                vec![self_2.node.clone(), cb.node.clone(), cc.node.clone(), cd.node.clone(), ce.node.clone()],
                || {}
            )
        }
    }

    pub fn lift6<B,C,D,E,F,G,FN: IsLambda6<A,B,C,D,E,F,G> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, ce: Cell<E>, cf: Cell<F>, f: FN) -> Cell<G> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static, E: Clone + Trace + Finalize + 'static, F: Clone + Trace + Finalize + 'static, G: Clone + Trace + Finalize + 'static {
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
            let cc = cc.clone();
            let cd = cd.clone();
            let ce = ce.clone();
            let cf = cf.clone();
            latch = gc_ctx.new_gc(UnsafeCell::new(Latch::new(
                move || {
                    let self_ = self_.clone();
                    let cb = cb.clone();
                    let cc = cc.clone();
                    let cd = cd.clone();
                    let ce = ce.clone();
                    let cf = cf.clone();
                    let f = f.clone();
                    MemoLazy::new(move || {
                        f.apply(&self_.sample_no_trans(), &cb.sample_no_trans(), &cc.sample_no_trans(), &cd.sample_no_trans(), &ce.sample_no_trans(), &cf.sample_no_trans())
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
                    return true;
                },
                update_deps,
                vec![self_2.node.clone(), cb.node.clone(), cc.node.clone(), cd.node.clone(), ce.node.clone()],
                || {}
            )
        }
    }

    pub fn switch_s(csa: Cell<Stream<A>>) -> Stream<A> {
        unimplemented!();
    }

    pub fn switch_c(cca: Cell<Cell<A>>) -> Cell<A> {
        unimplemented!();
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
                return true;
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

impl<A: Trace> Trace for Cell<A> {
    fn trace(&self, f: &mut FnMut(&GcDep)) {
        self.node.trace(f);
    }
}

impl<A: Finalize> Finalize for Cell<A> {
    fn finalize(&mut self) {
        self.node.finalize();
    }
}