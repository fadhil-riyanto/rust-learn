mod ret;
mod fmt;
mod impl_keyword;
mod scope;
mod primitive;

fn main() {
    let a: u32 = 332;
    let b: u32 = 39789;

    let z: u64 = ret::add::uint64_add(2, 4);

    println!("Hello, world!, im {}", z);
    fmt::full::do_full_fmt_stuff();
    impl_keyword::format::do_impl_debug_test();
    println!("ret callable: {}", impl_keyword::pure::callable());

    scope::mutability::scope_mutability();

    primitive::tuple::primitive();
}
