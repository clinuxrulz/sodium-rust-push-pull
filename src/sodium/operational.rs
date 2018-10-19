use sodium::Cell;
use sodium::Stream;
use sodium::impl_;

pub struct Operational {}

impl Operational {
    pub fn value<A>(ca: Cell<A>) -> Stream<A> {
        Stream {
            impl_: impl_::Operational::value(ca.impl_)
        }
    }

    pub fn updates<A>(ca: Cell<A>) -> Stream<A> {
        Stream {
            impl_: impl_::Operational::updates(ca.impl_)
        }
    }

    pub fn defer<A>(sa: Stream<A>) -> Stream<A> {
        Stream {
            impl_: impl_::Operational::defer(sa.impl_)
        }
    }

    pub fn split<C,A>(s: Stream<C>) -> Stream<A>
        where A: Clone + 'static,
              C: IntoIterator<Item=A> + 'static + Clone
    {
        Stream {
            impl_: impl_::Operational::split(s.impl_)
        }
    }
}
