use sodium::impl_::Cell;
use sodium::impl_::Stream;

pub struct Operational {}

impl Operational {
    pub fn value<A>(ca: Cell<A>) -> Stream<A> {
        unimplemented!();
    }

    pub fn updates<A>(ca: Cell<A>) -> Stream<A> {
        unimplemented!();
    }

    pub fn defer<A>(sa: Stream<A>) -> Stream<A> {
        unimplemented!();
    }

    pub fn split<C,A>(s: Stream<C>) -> Stream<A>
        where A: Clone + 'static,
              C: IntoIterator<Item=A> + 'static + Clone
    {
        unimplemented!();
    }
}