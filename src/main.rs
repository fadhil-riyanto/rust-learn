mod ret;
mod fmt;

fn main() {
    let a: u32 = 332;
    let b: u32 = 39789;

    let z: u64 = ret::add::uint64_add(2, 4);

    println!("Hello, world!, im {}", z);
    fmt::full::do_full_fmt_stuff();
    // fmt::full::
}
