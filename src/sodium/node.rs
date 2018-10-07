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
use std::rc::Weak;
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
    dependencies: Vec<Node>,
    dependents: Vec<WeakNode>,
    weak_sodium_ctx: WeakSodiumCtx
}

impl Node {
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

impl Trace for Node {
    fn trace(&self, f: &mut FnMut(&GcDep)) {
        let self_ = unsafe { &*(*self).data.get() };
        self_.dependencies.trace(f);
    }
}

impl Finalize for Node {
    fn finalize(&mut self) {
        let self_ = unsafe { &mut *(*self).data.get() };
        self_.dependencies.finalize();
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        let self_ = unsafe { &*(*self).data.get() };
        let other = unsafe { &*(*other).data.get() };
        self_.rank.cmp(&other.rank)
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
