use sodium::IsLambda1;
use sodium::Listener;
use sodium::SodiumCtx;
use sodium::gc::Finalize;
use sodium::gc::Trace;
use sodium::impl_;

pub struct Stream<A> {
    pub impl_: impl_::Stream<A>
}

impl<A: Clone + Trace + Finalize + 'static> Stream<A> {

    pub fn map<B: Clone + Trace + Finalize + 'static,F:IsLambda1<A,B> + 'static>(
        &self,
        f: F
    ) -> Stream<B> {
        Stream {
            impl_: self.impl_.map(f)
        }
    }

    pub fn filter<PRED:IsLambda1<A,bool> + 'static>(&self, pred: PRED) -> Stream<A> {
        Stream {
            impl_: self.impl_.filter(pred)
        }
    }

    pub fn listen<CALLBACK:FnMut(&A)+'static>(
        &self,
        callback: CALLBACK
    ) -> Listener {
        self.impl_.listen(callback)
    }
}
