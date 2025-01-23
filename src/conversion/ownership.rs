pub fn ownership_calle()
{
        let x = String::from("jwehkjwfeb");
        println!("{:p} addr [{}]", &x, x);
        take_ownership(x);
        // println!("{}", x);
}

fn take_ownership(xorig: String) -> ()
{
        // drop(xorig);
        println!("{:p} addr [{}]", &xorig, xorig);
}