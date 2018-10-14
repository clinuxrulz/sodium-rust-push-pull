use sodium::IsLambda1;
use sodium::IsLambda2;
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

    pub fn listen<CALLBACK:FnMut(&A)+'static>(
        &self,
        callback: CALLBACK
    ) -> Listener {
        self.impl_.listen(callback)
    }
}