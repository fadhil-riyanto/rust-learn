use std::result::Result;

fn divide(a: u32, b: u32) -> Result<u32, &'static str> {
        if b == 0 {
                return Result::Err("division by zero");
        } else {
                return Result::Ok(a / b);
        }
}       

fn propagation1() {
        let a: u32 = 18;
        let b: u32 = 0;

        let mut result = self::divide(a, b);
        match result {
                Result::Ok(x) => println!("result {x}"),
                Result::Err(x) => println!("ERR! {x}"),
                
        }
}

fn drink(str: &mut Option<&str>) -> Result<u32, &'static str>{
        if str.unwrap() == "still water".to_string() {
                println!("i like {}", str.unwrap());
                return Result::Ok(123);
        } else {
                return Result::Err("sorry, Im drinking still water only");
        }
}

fn drunnk(str: &mut Option<&str>) -> Result<u32, &'static str>{
        if str.unwrap() == "coffee".to_string() {
                println!("i like {}", str.unwrap());
                return Result::Ok(123);
        } else {
                return Result::Err("waduh");
        }
}

fn question() -> Result<u32, &'static str> {
        let mut a = Option::Some("still water");
        let mut ret: u32 = self::drink(&mut a)?; 

        let mut b = Option::Some("coffsee");
        ret = self::drunnk(&mut b)?; 

        return Ok(ret);
}


fn caller() {
        match self::question() {
                Ok(res) => println!("{}", res),
                Err(res) => println!("error: {}", res),
        }

}
pub fn do_propagation()
{
        self::propagation1();
        self::caller();
}