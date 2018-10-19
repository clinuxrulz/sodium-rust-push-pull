use sodium::Cell;
use sodium::CellLoop;
use sodium::CellSink;
use sodium::gc::Finalize;
use sodium::gc::Trace;

pub trait IsCell<A> {
    fn to_cell(self) -> Cell<A>;
}

impl<A> IsCell<A> for Cell<A> {
    fn to_cell(self) -> Cell<A> {
        self
    }
}

impl<A: Finalize + Trace + Clone + 'static> IsCell<A> for CellLoop<A> {
    fn to_cell(self) -> Cell<A> {
        Cell {
            impl_: self.impl_.to_cell()
        }
    }
}

impl<A: Finalize + Trace + Clone + 'static> IsCell<A> for CellSink<A> {
    fn to_cell(self) -> Cell<A> {
        Cell {
            impl_: self.impl_.to_cell()
        }
    }
}

impl<'r, A, CA: Clone + IsCell<A>> IsCell<A> for &'r CA {
    fn to_cell(self) -> Cell<A> {
        self.clone().to_cell()
    }
}
