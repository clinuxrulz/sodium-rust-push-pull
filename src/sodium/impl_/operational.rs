use sodium::impl_::gc::Finalize;
use sodium::impl_::gc::Trace;
use sodium::impl_::Cell;
use sodium::impl_::Stream;
use std::cell::UnsafeCell;
use std::rc::Rc;

pub struct Operational {}

impl Operational {
    pub fn value<A: Clone + Trace + Finalize + 'static>(ca: Cell<A>) -> Stream<A> {
        let sodium_ctx = ca.node.sodium_ctx();
        let sodium_ctx = &sodium_ctx;
        let deps = vec![ca.node.clone()];
        Stream::_new(
            sodium_ctx,
            move || {
                let next_value = unsafe { &*(*ca.next_value).get() };
                Some(next_value.clone())
            },
            deps,
            || {}
        )
    }

    pub fn updates<A: Clone + Trace + Finalize + 'static>(ca: Cell<A>) -> Stream<A> {
        let sodium_ctx = ca.node.sodium_ctx();
        let sodium_ctx = &sodium_ctx;
        let deps = vec![ca.node.clone()];
        let first = Rc::new(UnsafeCell::new(true));
        Stream::_new(
            sodium_ctx,
            move || {
                let first = unsafe { &mut *(*first).get() };
                if *first {
                    *first = false;
                    None
                } else {
                    let next_value = unsafe { &*(*ca.next_value).get() };
                    Some(next_value.clone())
                }
            },
            deps,
            || {}
        )
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