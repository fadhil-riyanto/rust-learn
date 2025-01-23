#[warn(unused_imports)]
use std::fmt;

struct Unprintable (i32);

#[derive(Debug)]

#[allow(dead_code)]
struct Printable (i32);

pub fn do_impl_debug_test()
{
        println!("test: {:?}", Printable(9));
}
