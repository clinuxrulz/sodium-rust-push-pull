use sodium::Dep;
use sodium::SodiumCtx;
use sodium::WeakSodiumCtx;
use sodium::gc::Finalize;
use sodium::gc::Gc;
use sodium::gc::GcDep;
use sodium::gc::GcWeak;
use sodium::gc::Trace;
use std::boxed::Box;
use std::cell::UnsafeCell;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::collections::HashSet;
use std::rc::Rc;
use std::vec::Vec;

pub struct Node {
    data: Gc<UnsafeCell<NodeData>>
}

pub struct WeakNode {
    data: GcWeak<UnsafeCell<NodeData>>
}

pub struct NodeData {
    id: u32,
    rank: u32,
    update: Box<FnMut()>,
    update_dependencies: Vec<Dep>,
    dependencies: Vec<Node>,
    dependents: Vec<WeakNode>,
    cleanup: Box<FnMut()>,
    weak_sodium_ctx: WeakSodiumCtx
}

impl Node {
    pub fn new<UPDATE: FnMut() + 'static, CLEANUP: FnMut() + 'static>(
        sodium_ctx: &SodiumCtx,
        update: UPDATE,
        update_dependencies: Vec<Dep>,
        dependencies: Vec<Node>,
        mut cleanup: CLEANUP
    ) -> Node {
        let id = sodium_ctx.new_id();
        let mut rank = 0;
        for dependency in &dependencies {
            let dependency = unsafe { &*(*dependency.data).get() };
            if rank <= dependency.rank {
                rank = dependency.rank + 1;
            }
        }
        let self_: Rc<UnsafeCell<Option<Node>>> = Rc::new(UnsafeCell::new(None));
        let cleanup2;
        {
            let self_ = self_.clone();
            cleanup2 = move || {
                cleanup();
                let self_ = unsafe { &mut *(*self_).get() };
                match self_ {
                    Some(ref mut self_) => {
                        let self_ = unsafe { &*(*self_.data).get() };
                        self_.dependencies.iter().for_each(|dependency| {
                            let dependency = unsafe { &mut *(*dependency.data).get() };
                            dependency.dependents.retain(|dependent| {
                                match dependent.upgrade() {
                                    Some(dependent) => {
                                        let dependent = unsafe { &*(*dependent.data).get() };
                                        dependent.id != self_.id
                                    },
                                    None => false
                                }
                            });
                        });
                    },
                    None => ()
                }
            };
        }
        let mut gc_ctx = sodium_ctx.gc_ctx();
        let node = Node {
            data: gc_ctx.new_gc(UnsafeCell::new(
                NodeData {
                    id,
                    rank,
                    update: Box::new(update),
                    update_dependencies,
                    dependencies: dependencies.clone(),
                    dependents: Vec::new(),
                    cleanup: Box::new(cleanup2),
                    weak_sodium_ctx: sodium_ctx.downgrade()
                }
            ))
        };
        unsafe {
            *(*self_).get() = Some(node.clone());
        };
        let weak_node = node.downgrade();
        for dependency in &dependencies {
            let dependency = unsafe { &mut *(*dependency.data).get() };
            dependency.dependents.push(weak_node.clone());
        }
        node
    }

    pub fn mark_dirty(&self) {
        self.mark_dirty2(&mut HashSet::new());
    }

    fn mark_dirty2(&self, visited: &mut HashSet<u32>) {
        let self_ = unsafe { &*(*self).data.get() };
        if visited.contains(&self_.id) {
            return;
        }
        visited.insert(self_.id);
        match self_.weak_sodium_ctx.upgrade() {
            Some(sodium_ctx) => {
                let sodium_ctx = unsafe { &mut *(*sodium_ctx.data).get() };
                sodium_ctx.to_be_updated.push(self.clone())
            },
            None => ()
        }
        self_.dependents.iter().for_each(|dependent| {
            dependent.upgrade().iter().for_each(|dependent| {
                dependent.mark_dirty2(visited);
            });
        });
    }

    pub fn ensure_bigger_than(&self, rank: u32) {
        self.ensure_bigger_than2(rank, &mut HashSet::new());
    }

    fn ensure_bigger_than2(&self, rank: u32, visited: &mut HashSet<u32>) {
        let self_ = unsafe { &mut *(*self).data.get() };
        if visited.contains(&self_.id) {
            return;
        }
        visited.insert(self_.id);
        if self_.rank <= rank {
            return
        }
        let rank2 = rank + 1;
        self_.rank = rank2;
        self_.dependents.iter().for_each(|dependent| {
            dependent.upgrade().iter().for_each(|dependent| {
                dependent.ensure_bigger_than2(rank2, visited);
            });
        })
    }

    pub fn update(&self) {
        let self_ = unsafe { &mut *(*self.data).get() };
        (self_.update)();
    }

    pub fn sodium_ctx(&self) -> SodiumCtx {
        let self_ = unsafe { &*(*self.data).get() };
        self_.weak_sodium_ctx.upgrade().unwrap()
    }

    pub fn downgrade(&self) -> WeakNode {
        WeakNode {
            data: self.data.downgrade()
        }
    }
}

impl WeakNode {
    pub fn upgrade(&self) -> Option<Node> {
        self.data.upgrade().map(|data| Node { data })
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            data: self.data.clone()
        }
    }
}

impl Clone for WeakNode {
    fn clone(&self) -> Self {
        WeakNode {
            data: self.data.clone()
        }
    }
}

impl Trace for Node {
    fn trace(&self, f: &mut FnMut(&GcDep)) {
        f(&self.data.to_dep());
    }
}

impl Finalize for Node {
    fn finalize(&mut self) {
    }
}

impl Trace for NodeData {
    fn trace(&self, f: &mut FnMut(&GcDep)) {
        self.dependencies.trace(f);
        self.update_dependencies.iter().for_each(|update_dep| f(&update_dep.gc_dep));
    }
}

impl Finalize for NodeData {
    fn finalize(&mut self) {
        (self.cleanup)();
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        let self_ = unsafe { &*(*self).data.get() };
        let other = unsafe { &*(*other).data.get() };
        self_.rank.cmp(&other.rank).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
