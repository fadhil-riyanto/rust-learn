struct Mydata {
        number: u32,
}

impl Mydata {
        fn nop() {
                println!("nop.... i'm cpu");
        }

        fn addq9(&mut self)
        {
                self.number = self.number + 9;
        }

        // this is void return
        fn print(&mut self) -> () {
                println!("number: {}", self.number)
        }

        fn ret(&mut self) -> u32 {
                return self.number;
        }
}

pub fn callable() -> u32
{
        let mut y = Mydata {
                number: 3,
        };
        y.addq9();
        return y.ret();
}