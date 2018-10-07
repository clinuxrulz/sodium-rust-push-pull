use sodium::Dep;
use sodium::Node;
use sodium::SodiumCtx;
use sodium::gc::Finalize;
use sodium::gc::Gc;
use sodium::gc::GcDep;
use sodium::gc::Trace;
use std::cell::UnsafeCell;
use std::vec::Vec;

pub struct Listener {
    data: Gc<UnsafeCell<ListenerData>>
}

struct ListenerData {
    unlisten: Option<Box<FnMut()>>,
    node: Node
}

impl Trace for ListenerData {
    fn trace(&self, f: &mut FnMut(&GcDep)) {
        self.node.trace(f);
    }
}

impl Finalize for ListenerData {
    fn finalize(&mut self) {
        self.node.finalize();
    }
}

impl Listener {
    pub fn new<UNLISTEN: FnMut() + 'static>(
        sodium_ctx: &SodiumCtx,
        unlisten: UNLISTEN,
        unlisten_dependencies: Vec<Dep>,
        dependencies: Vec<Node>
    ) -> Listener {
        let mut gc_ctx = sodium_ctx.gc_ctx();
        Listener {
            data: gc_ctx.new_gc(UnsafeCell::new(ListenerData {
                unlisten: Some(Box::new(unlisten)),
                node: Node::new(
                    sodium_ctx,
                    || {},
                    unlisten_dependencies,
                    dependencies,
                    || {}
                )
            }))
        }
    }

    pub fn unlisten(&self) {
        let self_ = unsafe { &mut *(*self.data).get() };
        match &mut self_.unlisten {
            Some(unlisten) => {
                unlisten();
            },
            None => ()
        }
        self_.unlisten = None;
    }
}
