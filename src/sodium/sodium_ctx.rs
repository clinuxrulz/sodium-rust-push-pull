use sodium::Cell;
use sodium::CellLoop;
use sodium::CellSink;
use sodium::Stream;
use sodium::StreamLoop;
use sodium::StreamSink;
use sodium::gc::Finalize;
use sodium::gc::GcCtx;
use sodium::gc::Trace;
use sodium::impl_;

pub struct SodiumCtx {
    impl_: impl_::SodiumCtx
}

impl SodiumCtx {
    pub fn new() -> SodiumCtx {
        SodiumCtx {
            impl_: impl_::SodiumCtx::new()
        }
    }

    pub fn new_cell<A: Clone + Trace + Finalize + 'static>(&self, value: A) -> Cell<A> {
        Cell {
            impl_: impl_::Cell::new(&self.impl_, value)
        }
    }

    pub fn new_stream<A: Clone + Trace + Finalize + 'static>(&self) -> Stream<A> {
        Stream {
            impl_: impl_::Stream::new(&self.impl_)
        }
    }

    pub fn new_cell_loop<A: Clone + Trace + Finalize + 'static>(&self) -> CellLoop<A> {
        CellLoop {
            impl_: impl_::CellLoop::new(&self.impl_)
        }
    }

    pub fn new_stream_loop<A: Clone + Trace + Finalize + 'static>(&self) -> StreamLoop<A> {
        StreamLoop {
            impl_: impl_::StreamLoop::new(&self.impl_)
        }
    }

    pub fn new_cell_sink<A: Clone + Trace + Finalize + 'static>(&self, value: A) -> CellSink<A> {
        CellSink {
            impl_: impl_::CellSink::new(&self.impl_, value)
        }
    }

    pub fn new_stream_sink<A: Clone + Trace + Finalize + 'static>(&self) -> StreamSink<A> {
        StreamSink {
            impl_: impl_::StreamSink::new(&self.impl_)
        }
    }

    pub fn gc_ctx(&self) -> GcCtx {
        self.impl_.gc_ctx()
    }

    pub fn transaction<A,CODE:FnOnce()->A>(&self, code: CODE) -> A {
        self.impl_.transaction(code)
    }
}