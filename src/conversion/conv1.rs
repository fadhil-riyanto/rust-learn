use std::convert::From;

// from "from" to "into"

pub fn simple_conv()
{
        tostr();
        scope();
        mv_datatype();
        strcpy();

}

fn scope()
{
        let mut x: u32 = 8;
        x = x + 1;

        {
                let x = x * 3;
                println!("{}", x);
        }

        println!("{}", x);
}

fn strcpy()
{
        #[allow(unused)]
        let mut s1 = String::from("wkwk");
        #[allow(unused)]
        let mut s2 = s1.clone();
        println!("{}", s1);
}

fn tostr()
{
        let mystr = String::from("wwwww");
        println!("{}", mystr);

}

fn mv_datatype()
{
        let mut a = "aaaa";
        let mut b = "ccccc";
        a = &b;

        println!("{}", &a);
}