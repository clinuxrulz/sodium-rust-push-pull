use sodium::Stream;
use sodium::StreamLoop;
use sodium::StreamSink;
use sodium::gc::Finalize;
use sodium::gc::Trace;

pub trait IsStream<A> {
    fn to_stream(self) -> Stream<A>;
}

impl<A> IsStream<A> for Stream<A> {
    fn to_stream(self) -> Stream<A> {
        self
    }
}

impl<A: Finalize + Trace + Clone + 'static> IsStream<A> for StreamLoop<A> {
    fn to_stream(self) -> Stream<A> {
        Stream {
            impl_: self.impl_.to_stream()
        }
    }
}

impl<A: Finalize + Trace + Clone + 'static> IsStream<A> for StreamSink<A> {
    fn to_stream(self) -> Stream<A> {
        Stream {
            impl_: self.impl_.to_stream()
        }
    }
}

impl<'r, A, SA: Clone + IsStream<A>> IsStream<A> for &'r SA {
    fn to_stream(self) -> Stream<A> {
        self.clone().to_stream()
    }
}
