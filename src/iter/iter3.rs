use crate::{dd, helper::debug};

pub fn handle_iter3() -> () {
        self::iter3_1();
        self::more_slice1();
        self::ptr_stuff();
        self::loop_iter();
}

fn loop_iter() -> () {
        // todo, find 8
        let x: [u8; 10] = [
                2, 3, 4, 1, 9, 2, 3, 4,8, 8
        ];

        let xiter = x.iter();

        // dd!(xiter.next());

        // for (i, &item) in x.iter().enumerate() {
        //         println!("{}", item);
        // }


}

pub fn ptr_stuff()
{
        // this is string section
        let a: String = String::from("aaaa");
        println!("{:p}", &a);

        let b: &String = &a;
        println!("{:p}", b);

        // this is ptr for array u8
        let z: [u8; 5] = [3, 4, 1, 2, 9];
        // dd!(z);
        let z_portion: &[u8] = &z[0..3];
        dd!(z_portion);

        // check ptr
        println!("{:p}", &z);

        let z_deref: &[u8] = &z;
        println!("{:p}", z_deref);
}

pub fn iter3_1()
{
        let mut x: String = String::from("wkwk");

        // let changed: &mut [u8] = &[0..2];
        // let a: &[u8] = x.as_bytes();

        dd!(&x[0..2]);
}

fn more_slice1() -> () { // this func show ref portion of string
        let mut x: String = String::from("doing some random stuff");

        let mut y: &str = &x[0..5];
        // dd!(y);
        strslice_ref(&y);
}

fn strslice_ref(data: &str) {
        dd!(data);
}