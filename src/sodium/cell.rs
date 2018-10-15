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

pub struct Cell<A> {
    pub impl_: impl_::Cell<A>
}

impl<A: Clone + Trace + Finalize + 'static> Cell<A> {

    pub fn map<B: Clone + Trace + Finalize + 'static,F:IsLambda1<A,B> + 'static>(
        &self,
        f: F
    ) -> Cell<B> {
        Cell {
            impl_: self.impl_.map(f)
        }
    }

    pub fn lift2<B,C,F: IsLambda2<A,B,C> + 'static>(&self, cb: Cell<B>, f: F) -> Cell<C> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static {
        Cell {
            impl_: self.impl_.lift2(cb.impl_, f)
        }
    }

    pub fn lift3<B,C,D,F: IsLambda3<A,B,C,D> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, f: F) -> Cell<D> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static {
        Cell {
            impl_: self.impl_.lift3(cb.impl_, cc.impl_, f)
        }
    }

    pub fn lift4<B,C,D,E,F: IsLambda4<A,B,C,D,E> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, f: F) -> Cell<E> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static, E: Clone + Trace + Finalize + 'static {
        Cell {
            impl_: self.impl_.lift4(cb.impl_, cc.impl_, cd.impl_, f)
        }
    }

    pub fn lift5<B,C,D,E,F,FN: IsLambda5<A,B,C,D,E,F> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, ce: Cell<E>, f: FN) -> Cell<F> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static, E: Clone + Trace + Finalize + 'static, F: Clone + Trace + Finalize + 'static {
        Cell {
            impl_: self.impl_.lift5(cb.impl_, cc.impl_, cd.impl_, ce.impl_, f)
        }
    }

    pub fn lift6<B,C,D,E,F,G,FN: IsLambda6<A,B,C,D,E,F,G> + 'static>(&self, cb: Cell<B>, cc: Cell<C>, cd: Cell<D>, ce: Cell<E>, cf: Cell<F>, f: FN) -> Cell<G> where B: Clone + Trace + Finalize + 'static, C: Clone + Trace + Finalize + 'static, D: Clone + Trace + Finalize + 'static, E: Clone + Trace + Finalize + 'static, F: Clone + Trace + Finalize + 'static, G: Clone + Trace + Finalize + 'static {
        Cell {
            impl_: self.impl_.lift6(cb.impl_, cc.impl_, cd.impl_, ce.impl_, cf.impl_, f)
        }
    }

    pub fn listen<CALLBACK:FnMut(&A)+'static>(
        &self,
        callback: CALLBACK
    ) -> Listener {
        self.impl_.listen(callback)
    }
}