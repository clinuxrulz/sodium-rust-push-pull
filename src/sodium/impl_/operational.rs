use sodium::impl_::gc::Finalize;
use sodium::impl_::gc::Gc;
use sodium::impl_::gc::Trace;
use sodium::impl_::Cell;
use sodium::impl_::MemoLazy;
use sodium::impl_::Node;
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

    pub fn defer<A: Clone + Trace + Finalize + 'static>(sa: Stream<A>) -> Stream<A> {
        Operational::split(sa.map(|a:&A| vec![a.clone()]))
    }

    pub fn split<C,A>(s: Stream<C>) -> Stream<A>
        where A: Clone + Trace + Finalize + 'static,
              C: IntoIterator<Item=A> + Clone + Trace + Finalize + 'static
    {
        let sodium_ctx = s.node.sodium_ctx();
        let sodium_ctx = &sodium_ctx;
        let mut gc_ctx = sodium_ctx.gc_ctx();
        let gc_ctx = &mut gc_ctx;
        let deps = vec![s.node.clone()];
        let value: Gc<UnsafeCell<Option<MemoLazy<A>>>> = gc_ctx.new_gc(UnsafeCell::new(None));
        let sodium_ctx2 = sodium_ctx.clone();
        let node2;
        {
            let value = value.clone();
            node2 = Node::new(
                sodium_ctx,
                move || {
                    let sodium_ctx = &sodium_ctx2;
                    let value = value.clone();
                    sodium_ctx.post(move || {
                        let value = unsafe { &mut *(*value).get() };
                        *value = None;
                    });
                    true
                },
                Vec::new(),
                Vec::new(),
                || {}
            );
        }
        let result = Stream {
            value: value.clone(),
            node: node2.clone()
        };
        let node2_dep = node2.to_dep();
        let sodium_ctx2 = sodium_ctx.clone();
        let node1;
        {
            let node2 = node2.clone();
            node1 = Node::new(
                sodium_ctx,
                move || {
                    let sodium_ctx = &sodium_ctx2;
                    let sodium_ctx2 = sodium_ctx.clone();
                    let node2 = node2.clone();
                    let s_value_op = s.peek_value();
                    let value = value.clone();
                    if let Some(s_value) = s_value_op {
                        sodium_ctx.post(move || {
                            let sodium_ctx = &sodium_ctx2;
                            let node2 = node2.clone();
                            let s_value = s_value.clone();
                            let value = unsafe { &mut *(*value).get() };
                            s_value.get().clone().into_iter().for_each(move |a| {
                                let sodium_ctx2 = sodium_ctx.clone();
                                sodium_ctx.transaction(|| {
                                    let sodium_ctx = &sodium_ctx2;
                                    let a = a.clone();
                                    *value = Some(sodium_ctx.new_lazy(move || a.clone()));
                                    node2.mark_dependents_dirty();
                                });
                            });
                        });
                    }
                    false
                },
                vec![node2_dep],
                deps,
                || {}
            );
        }
        node2.add_dependencies(vec![node1]);
        result
    }
}