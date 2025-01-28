use std::num::ParseIntError;
use crate::errors::myrandom;

#[derive(Debug)]
enum TypePeople {
        Male(u32),
        Female(u32)
}

fn enum_param() -> ()
{
        let mut y = TypePeople::Female(18);
        let TypePeople::Female(c) = y else {
                println!("oops");
                panic!("oopsd");
        };
        // let TypePeople::Male(c) = y;

        // match y {
        //         TypePeople::Female(c) => println!("{}", c),
        //         TypePeople::Male(c) => println!("{}", c),
        // }

        // let TypePeople::Female(d) = y;
        dbg!(y);
        ()
}

fn fr_question_mark() -> () {
        let mut y: Result<i32, ParseIntError> = "283".parse();
        let Result::Ok(c) = y else {
                eprintln!("ERRR {}", y.err().unwrap());
                panic!("okokkok");
        };

        dbg!(c);
}

fn err_handler() -> () {
        let b: Result<i32, ParseIntError> = i32::from_str_radix("12", 10);
        if let Err(e) = &b {
                println!("Failed conversion to i32: {e}");
        }

        if let Ok(e) = &b {
                println!("result: {e}");
        }
}

fn divide(a: u32, b: u32) -> Option<u32>
{
        if b == 0 {
                return Option::None;
        } else {
                return Option::Some(a / b);
        }
}

fn divide_generic<T>(a: T, b: T) -> Option<T>
where  
        T: PartialEq<u32> + std::ops::Div<Output = T>
{
        if b == 0 {
                return Option::None;
        } else {
                return Option::Some(a / b);
        }
}
fn optional_value()
{
        let a: u32 = 50;
        let b: u32 = 25;

        let mut y: Option<u32> = self::divide(a, b);
        if let Option::Some(c) = &y {
                println!("{c}")
        } else {
                println!("error, division by zero")
        }
}

pub fn question_mark()
{
        enum_param();
        fr_question_mark();
        err_handler();
        optional_value();
        // random::callee();
}