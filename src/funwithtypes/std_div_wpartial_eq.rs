use std::ops::Div;

#[derive(Debug)]
struct MyDiv {
        a: u32,
        b: u32,
        ret: 32,
}

impl Div for MyDiv {
        type Output = Self;


        fn div(self, rhs: Self) -> Self::Output {
                MyDiv {
                        a: self.a,
                        b: rhs.b,
                        ret: self.a / rhs.b
                }
        }
}


pub fn fun()
{
        // let c = 

        
}