use sodium::Cell;
use sodium::Dep;
use sodium::IsLambda1;
use sodium::IsLambda2;
use sodium::IsLambda3;
use sodium::IsLambda4;
use sodium::IsLambda5;
use sodium::IsLambda6;
use sodium::Listener;
use sodium::gc::Finalize;
use sodium::gc::Trace;
use sodium::impl_;

pub struct Stream<A> {
    pub impl_: impl_::Stream<A>
}

impl<A: Clone + Trace + Finalize + 'static> Stream<Option<A>> {
    pub fn filter_option(&self) -> Stream<A> {
        Stream {
            impl_: self.impl_.filter_option()
        }
    }
}

impl<A: Clone + Trace + Finalize + 'static> Stream<A> {

    pub fn to_dep(&self) -> Dep {
        self.impl_.to_dep()
    }

    pub fn map<B: Clone + Trace + Finalize + 'static,F:IsLambda1<A,B> + 'static>(
        &self,
        f: F
    ) -> Stream<B> {
        Stream {
            impl_: self.impl_.map(f)
        }
    }

    pub fn hold(&self, a: A) -> Cell<A> {
        Cell {
            impl_: self.impl_.hold(a)
        }
    }

    pub fn filter<PRED:IsLambda1<A,bool> + 'static>(&self, pred: PRED) -> Stream<A> {
        Stream {
            impl_: self.impl_.filter(pred)
        }
    }

    pub fn snapshot<B>(&self, cb: Cell<B>) -> Stream<B> where B: Trace + Finalize + Clone + 'static {
        Stream {
            impl_: self.impl_.snapshot(cb.impl_)
        }
    }

    pub fn snapshot2<B,C,FN:IsLambda2<A,B,C> + 'static>(&self, cb: Cell<B>, f: FN) -> Stream<C> where B: Trace + Finalize + Clone + 'static, C: Trace + Finalize + Clone + 'static {
        Stream {
            impl_: self.impl_.snapshot2(cb.impl_, f)
        }
    }

    pub fn snapshot3<B,C,D,FN:IsLambda3<A,B,C,D> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, f: FN) -> Stream<D> where B: Trace + Finalize + Clone + 'static, C: Trace + Finalize + Clone + 'static, D: Trace + Finalize + Clone + 'static {
        Stream {
            impl_: self.impl_.snapshot3(cb.impl_, cc.impl_, f)
        }
    }

    pub fn snapshot4<B,C,D,E,FN:IsLambda4<A,B,C,D,E> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, f: FN) -> Stream<E> where B: Trace + Finalize + Clone + 'static, C: Trace + Finalize + Clone + 'static, D: Trace + Finalize + Clone + 'static, E: Trace + Finalize + Clone + 'static {
        Stream {
            impl_: self.impl_.snapshot4(cb.impl_, cc.impl_, cd.impl_, f)
        }
    }

    pub fn snapshot5<B,C,D,E,F,FN:IsLambda5<A,B,C,D,E,F> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, ce: Cell<E>, f: FN) -> Stream<F> where B: Trace + Finalize + Clone + 'static, C: Trace + Finalize + Clone + 'static, D: Trace + Finalize + Clone + 'static, E: Trace + Finalize + Clone + 'static, E: Trace + Finalize + Clone + 'static, F: Trace + Finalize + Clone + 'static {
        Stream {
            impl_: self.impl_.snapshot5(cb.impl_, cc.impl_, cd.impl_, ce.impl_, f)
        }
    }

    pub fn snapshot6<B,C,D,E,F,G,FN:IsLambda6<A,B,C,D,E,F,G> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, ce: Cell<E>, cf: Cell<F>, f: FN) -> Stream<G> where B: Trace + Finalize + Clone + 'static, C: Trace + Finalize + Clone + 'static, D: Trace + Finalize + Clone + 'static, E: Trace + Finalize + Clone + 'static, E: Trace + Finalize + Clone + 'static, F: Trace + Finalize + Clone + 'static, G: Trace + Finalize + Clone + 'static {
        Stream {
            impl_: self.impl_.snapshot6(cb.impl_, cc.impl_, cd.impl_, ce.impl_, cf.impl_, f)
        }
    }

    pub fn listen<CALLBACK:FnMut(&A)+'static>(
        &self,
        callback: CALLBACK
    ) -> Listener {
        self.impl_.listen(callback)
    }
}

impl<A: Clone + Trace + Finalize + 'static> Clone for Stream<A> {
    fn clone(&self) -> Self {
        Stream {
            impl_: self.impl_.clone()
        }
    }
}
