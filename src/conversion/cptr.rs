pub fn do_ptr_copy()
{
        let a: u32 = 8312987;
        let b: *const u32 = &a;

        unsafe {
                println!("{}", *b.offset(8));
        }
}