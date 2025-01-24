// todo: copy a number using pointer, if possible also use unsafe

pub fn do_copy_int()
{
        let mut a: u32 = 83838;
        do_unsafe_change_ptr(&mut a);
        let static_new_int: u32 = do_safe_change_ptr(a); // changed ownership here
        println!("{static_new_int}");
}

fn do_unsafe_change_ptr(x: &mut u32) -> () {
        unsafe {
                *x = 392;
        }
}

fn do_safe_change_ptr(mut x: u32) -> u32 {
        x = 2332;
        return x;
}