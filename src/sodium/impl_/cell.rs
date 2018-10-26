use sodium::impl_::Dep;
use sodium::impl_::Lambda;
use sodium::impl_::IsLambda0;
use sodium::impl_::IsLambda1;
use sodium::impl_::IsLambda2;
use sodium::impl_::IsLambda3;
use sodium::impl_::IsLambda4;
use sodium::impl_::IsLambda5;
use sodium::impl_::IsLambda6;
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
    pub value: Gc<UnsafeCell<MemoLazy<A>>>,
    pub next_value: Gc<UnsafeCell<Option<MemoLazy<A>>>>,
    pub node: Node
}

impl<A: Clone + Trace + Finalize + 'static> Cell<A> {
    pub fn new(sodium_ctx: &SodiumCtx, value: A) -> Cell<A> {
        Cell::_new(
            sodium_ctx,
            MemoLazy::new(move || value.clone()),
            || None,
            Vec::new(),
            || {}
        )
    }

    pub fn _new<UPDATE:IsLambda0<Option<MemoLazy<A>>>+'static, CLEANUP: FnMut()+'static>(
        sodium_ctx: &SodiumCtx,
        init_value: MemoLazy<A>,
        update: UPDATE,
        deps: Vec<Node>,
        cleanup: CLEANUP
    ) -> Cell<A> {
        let mut gc_ctx = sodium_ctx.gc_ctx();
        let value = gc_ctx.new_gc(UnsafeCell::new(init_value));
        let next_value = gc_ctx.new_gc(UnsafeCell::new(None));
        let update_deps = update.deps();
        let sodium_ctx2 = sodium_ctx.clone();
        Cell {
            value: value.clone(),
            next_value: next_value.clone(),
            node: Node::new(
                sodium_ctx,
                move || {
                    let sodium_ctx = sodium_ctx2.clone();
                    let sodium_ctx = &sodium_ctx;
                    let val_op = update.apply();
                    let next_value2 = unsafe { &mut *(*next_value).get() };
                    let val_op_is_some = val_op.is_some();
                    if let Some(val) = val_op {
                        *next_value2 = Some(val);
                        let value = value.clone();
                        let next_value = next_value.clone();
                        sodium_ctx.post(move || {
                            let value = unsafe { &mut *(*value).get() };
                            let next_value = unsafe { &mut *(*next_value).get() };
                            if let &mut Some(ref val) = next_value {
                                *value = val.clone();
                            }
                            *next_value = None;
                        });
                    } else {
                        *next_value2 = None;
                    }
                    val_op_is_some
                },
                update_deps,
                deps,
                cleanup
            )
        }
    }

    pub fn to_dep(&self) -> Dep {
        self.node.to_dep()
    }

    pub fn sample_no_trans(&self) -> A {
        let thunk = unsafe { &*(*self.value).get() };
        thunk.get().clone()
    }

    pub fn _next_value_thunk_op(&self) -> Option<MemoLazy<A>> {
        let thunk_op = unsafe { &*(self.next_value).get() };
        thunk_op.clone()
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
        let self_ = self.clone();
        let update_deps = f.deps();
        let f = Rc::new(f);
        let init_value;
        {
            let self_ = self.clone();
            let f = f.clone();
            init_value = MemoLazy::new(move || {
                f.apply(&self_.sample_no_trans())
            });
        }
        let node_deps = vec![self_.node.clone()];
        Cell::_new(
            sodium_ctx,
            init_value,
            Lambda::new(move || {
                let self_ = self_.clone();
                let f = f.clone();
                Some(MemoLazy::new(move || f.apply(&self_.sample_no_trans())))
            }, update_deps),
            node_deps,
            || {}
        )
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
        let ca = self.clone();
        let node_deps = vec![ca.node.clone(), cb.node.clone()];
        let init_value;
        {
            let f = f.clone();
            let ca = ca.clone();
            let cb = cb.clone();
            init_value = MemoLazy::new(move || {
                f.apply(&ca.sample_no_trans(), &cb.sample_no_trans())
            })
        }
        let update = Lambda::new(
            move || {
                if let Some(a_thunk) = ca._next_value_thunk_op() {
                    if let Some(b_thunk) = cb._next_value_thunk_op() {
                        let f = f.clone();
                        return Some(MemoLazy::new(move || {
                            f.apply(a_thunk.get(), b_thunk.get())
                        }));
                    }
                }
                None
            },
            update_deps
        );
        Cell::_new(
            sodium_ctx,
            init_value,
            update,
            node_deps,
            || {}
        )
    }

    pub fn lift3<B,C,D,F: IsLambda3<A,B,C,D> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, f: F) -> Cell<D> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static {
        let update_deps = f.deps();
        self
            .lift2(
                cb,
                |a: &A, b: &B| (a.clone(), b.clone())
            )
            .lift2(
                cc,
                Lambda::new(
                    move |a_b: &(A,B), c: &C| {
                        let &(ref a, ref b) = a_b;
                        f.apply(a, b, c)
                    },
                    update_deps
                )
            )
    }

    pub fn lift4<B,C,D,E,F: IsLambda4<A,B,C,D,E> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, f: F) -> Cell<E> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static, E: Clone + Trace + Finalize + 'static {
        let update_deps = f.deps();
        self
            .lift2(
                cb,
                |a: &A, b: &B| (a.clone(), b.clone())
            )
            .lift3(
                cc,
                cd,
                Lambda::new(
                    move |a_b: &(A,B), c: &C, d: &D| {
                        let &(ref a, ref b) = a_b;
                        f.apply(a, b, c, d)
                    },
                    update_deps
                )
            )
    }

    pub fn lift5<B,C,D,E,F,FN: IsLambda5<A,B,C,D,E,F> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, ce: Cell<E>, f: FN) -> Cell<F> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static, E: Clone + Trace + Finalize + 'static, F: Clone + Trace + Finalize + 'static {
        let update_deps = f.deps();
        self
            .lift3(
                cb,
                cc,
                |a: &A, b: &B, c: &C| ((a.clone(), b.clone()), c.clone())
            )
            .lift3(
                cd,
                ce,
                Lambda::new(
                    move |a_b_c: &((A,B),C), d: &D, e: &E| {
                        let &((ref a, ref b), ref c) = a_b_c;
                        f.apply(a, b, c, d, e)
                    },
                    update_deps
                )
            )
    }

    pub fn lift6<B,C,D,E,F,G,FN: IsLambda6<A,B,C,D,E,F,G> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, ce: Cell<E>, cf: Cell<F>, fn_: FN) -> Cell<G> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static, E: Clone + Trace + Finalize + 'static, F: Clone + Trace + Finalize + 'static, G: Clone + Trace + Finalize + 'static {
        let update_deps = fn_.deps();
        self
            .lift3(
                cb,
                cc,
                |a: &A, b: &B, c: &C| ((a.clone(), b.clone()), c.clone())
            )
            .lift4(
                cd,
                ce,
                cf,
                Lambda::new(
                    move |a_b_c: &((A,B),C), d: &D, e: &E, f: &F| {
                        let &((ref a, ref b), ref c) = a_b_c;
                        fn_.apply(a, b, c, d, e, f)
                    },
                    update_deps
                )
            )
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
                let val = self_.sample_no_trans();
                (*callback)(&val);
            });
        }
        Listener::new(Node::new(
            sodium_ctx,
            move || {
                let callback = unsafe { &mut *(*callback).get() };
                let val = self_._next_value_thunk_op().map(|thunk| thunk.get().clone()).unwrap_or_else(|| self_.sample_no_trans());
                (*callback)(&val);
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
            next_value: self.next_value.clone(),
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
