use sodium::Cell;
use sodium::gc::Finalize;
use sodium::gc::Trace;
use sodium::impl_;

pub struct CellLoop<A> {
    pub impl_: impl_::CellLoop<A>
}

impl<A: Trace + Finalize + Clone + 'static> CellLoop<A> {

    pub fn loop_(&self, ca: Cell<A>) {
        self.impl_.loop_(ca.impl_);
    }

    pub fn to_cell(&self) -> Cell<A> {
        Cell {
            impl_: self.impl_.to_cell()
        }
    }
}
