#![allow(warnings)]

mod iter;
mod ptr_in_rust;
mod conversion;
mod ret;
mod fmt;
mod impl_keyword;
mod scope;
mod primitive;

fn main() {
    #[allow(identio)]
    let a: u32 = 332;
    let b: u32 = 39789;

    let z: u64 = ret::add::uint64_add(2, 4);

    println!("Hello, world!, im {}", z);
    fmt::full::do_full_fmt_stuff();
    impl_keyword::format::do_impl_debug_test();
    println!("ret callable: {}", impl_keyword::pure::callable());

    scope::mutability::scope_mutability();

    primitive::tuple::primitive();
    conversion::conv1::simple_conv();
    conversion::ownership::ownership_calle();
    conversion::cptr::do_ptr_copy();

    conversion::ownership2::ownership2();

    ptr_in_rust::ptr1::ptrissame();
    ptr_in_rust::copying::do_copy_int();
    ptr_in_rust::double_ref::double_ref();
    ptr_in_rust::double_ref::double_ref_immutability();

    iter::iter1::iter_enumerate_test();
}
