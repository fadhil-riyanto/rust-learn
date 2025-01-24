pub fn ptrissame()
{
        // /**
        //  * todo: make sure ptr is same accross function call
        //  */

        let a: String = String::from("aaaaa");
        // let b: [char; 5] = a.ptr;
        // println!("{}", b);
        println!("{:p}", &a);
        ptrstr(&a);

        do_static_change_ptr();

        let mut y: String = String::from("wkwk");
        do_change_ptr_fn(&mut y);

        println!("{y}");

}
fn do_change_ptr_fn(tu: &mut String) {
        tu.push_str("aaaa");
}

fn do_static_change_ptr()
{
        let mut x: String = String::from("aaaa");
        x.push_str("ok");

        println!("{x}");
}

fn ptrstr(x: &String) -> () {
        println!("{:p}", x)
}