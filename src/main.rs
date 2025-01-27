#![allow(warnings)]

mod iter;
mod vec;
mod ptr_in_rust;
mod conversion;
mod ret;
mod fmt;
mod impl_keyword;
mod scope;
mod primitive;
mod helper;
mod e_trait;

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
    iter::iter2::do_iter2();
    
    e_trait::trait1::trait1();
    iter::iter3::handle_iter3();

    vec::vec1::do_vec_stuff();
    vec::vec_chr::do_vec_chr();
    vec::panic::do_panic();
}
