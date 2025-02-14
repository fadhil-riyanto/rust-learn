use crate::rust_by_example::freezing;
use crate::rust_by_example::casting;
use crate::rust_by_example::tryinto;
use crate::rust_by_example::default_type;
use crate::rust_by_example::asref;
use crate::rust_by_example::trait_bound;
use crate::rust_by_example::iter_sugar;
use crate::rust_by_example::buffer;
use crate::rust_by_example::heap;
use crate::rust_by_example::mymacro;
// use super::asref::do_asref;

pub fn run_me() -> () {
        freezing::freezing();
        casting::casting();
        tryinto::tryinto_main();
        asref::do_asref();
        trait_bound::dotrait();
        iter_sugar::run_sugar();
        buffer::do_reserve();
        heap::test_box();
        mymacro::testmacro();
}