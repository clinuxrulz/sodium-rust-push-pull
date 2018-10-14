use sodium::Cell;
use sodium::CellSink;
use sodium::SodiumCtx;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn map_c() {
    let mut sodium_ctx = SodiumCtx::new();
    let sodium_ctx = &mut sodium_ctx;
    {
        let c = sodium_ctx.new_cell_sink(6);
        let c2 = c.to_cell();
        let out = Rc::new(RefCell::new(Vec::new()));
        let l;
        {
            let out = out.clone();
            l = c2.map(|a: &i32| format!("{}", a)).listen(
                move |a|
                    out.borrow_mut().push(a.clone())
            );
        }
        c.send(8);
        l.unlisten();
        assert_eq!(vec![String::from("6"), String::from("8")], *out.borrow());
    }
    //assert_memory_freed(sodium_ctx);
}
