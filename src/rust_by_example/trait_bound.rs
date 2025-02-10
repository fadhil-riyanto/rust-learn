use std::fmt::{self, write};

use rand::Rng; 
use chrono;

struct Msg {
        text: String,
        date: u32,
        owner: String
}


pub trait MsgStuff {
        fn getOwner(&self) -> String;
        fn getdate(&self) -> ();
        fn default_impl(&self) -> () {
                println!("you call me");
        }
}

trait MyRandom {
        fn getMyRandom(&self) -> i32
        {
                return rand::thread_rng().gen_range(0..1000)
        }
}

impl MsgStuff for Msg {
        fn getOwner(&self) -> String {
            return self.owner.clone()
        }

        fn getdate(&self) -> () {
                println!("{:?}", chrono::offset::Local::now());
        }

        
}

impl fmt::Display for Msg {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "owner is {}", self.owner)
        }
}

// impl myran

// trait as parameters
// recv any implementation of msgstuff
fn trait_as_params(x: &impl MsgStuff) {
        x.getdate();
        x.default_impl();
}

fn trait_as_param_but_verbose<T: MsgStuff>(x: &T) {
        x.getdate();
}

fn doing_formatter(x: &impl fmt::Display) {
        println!("{}", format!("oriign: {x}"))
}

fn doing_formatter_and_other(x: &(impl fmt::Display + MsgStuff)) -> () {
        println!("{}", format!("sjjsjs: {x}"));
        x.getdate();
        x.default_impl();
}


fn doing_formatter_and_other_verbose<T: fmt::Display + MsgStuff>(x: &T) -> () {
        println!("{}", format!("sjjsjs: {x}"));
        x.getdate();
        x.default_impl();
}

fn exp_slice()
{
        let xs: [i32; 5] = [1, 2, 3, 4, 5];
        let y = &[xs];

        let z: &[i32; 3] = &[2, 3, 4];
        println!("{:?}", z);
}


pub fn dotrait()
{
        let message: Msg = Msg{
                text: "lol".to_string(),
                date: 27382,
                owner: "faddhil".to_string()
        };

        message.getdate();
        // message.default_impl();
        // trait_as_params(&message);
        // trait_as_param_but_verbose(&message);
        // doing_formatter(&message);
        // doing_formatter_and_other(&message);
        doing_formatter_and_other_verbose(&message);
        exp_slice();
}