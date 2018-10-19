use sodium::Stream;
use sodium::gc::Finalize;
use sodium::gc::Trace;
use sodium::impl_;

pub struct StreamLoop<A> {
    pub impl_: impl_::StreamLoop<A>
}

impl<A: Trace + Finalize + Clone + 'static> StreamLoop<A> {

    pub fn loop_(&self, sa: Stream<A>) {
        self.impl_.loop_(sa.impl_);
    }

    pub fn to_stream(&self) -> Stream<A> {
        Stream {
            impl_: self.impl_.to_stream()
        }
    }
}
