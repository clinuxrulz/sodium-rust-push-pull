use sodium::Cell;
use sodium::gc::Finalize;
use sodium::gc::Trace;
use sodium::impl_;

pub struct CellSink<A> {
    pub impl_: impl_::CellSink<A>
}

impl<A: Clone + Trace + Finalize + 'static> CellSink<A> {
    pub fn send(&self, a: A) {
        self.impl_.send(a);
    }

    pub fn to_cell(&self) -> Cell<A> {
        Cell {
            impl_: self.impl_.to_cell()
        }
    }
}
