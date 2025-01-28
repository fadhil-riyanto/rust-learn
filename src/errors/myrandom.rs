// this section explain about partial equality
use std::cmp::PartialEq;
#[derive(PartialEq)]
enum Gender {
        Male,
        Female,
}


struct IMHuman {
        age: u32,
        outtype: Gender,
}

impl PartialEq for IMHuman {
        fn eq(&self, other: &Self) -> bool {
                self.outtype == other.outtype
        }
}

pub fn callee() -> () {
        
}