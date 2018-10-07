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
use std::vec::Vec;

pub struct Node {
    rank: u32,
    dirty: bool,
    update: Box<FnMut()>,
    dependencies: Vec<Gc<UnsafeCell<Node>>>,
    dependents: Vec<GcWeak<UnsafeCell<Node>>>
}

impl Trace for Node {
    fn trace(&self, f: &mut FnMut(&GcDep)) {
        self.dependencies.iter().for_each(|dependency| unsafe { (*(**dependency).get()).trace(f) });
    }
}

impl Finalize for Node {
    fn finalize(&mut self) {
        self.dependencies.iter().for_each(|dependency| unsafe { (*(**dependency).get()).finalize() });
    }
}


impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.rank.cmp(&other.rank)
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
