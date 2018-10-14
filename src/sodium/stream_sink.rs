use sodium::Stream;
use sodium::gc::Finalize;
use sodium::gc::Trace;
use sodium::impl_;

pub struct StreamSink<A> {
    pub impl_: impl_::StreamSink<A>
}

impl<A: Clone + Trace + Finalize + 'static> StreamSink<A> {
    pub fn send(&self, a: A) {
        self.impl_.send(a);
    }

    pub fn to_stream(&self) -> Stream<A> {
        Stream {
            impl_: self.impl_.to_stream()
        }
    }
}
