/**
 * this section use how to explain mut keyword
 */

#[derive(Default)]
struct Stack {
        number: u32,
}

impl Stack {
        fn setq(&mut self, data: u32) -> () {
                self.number = data;
        }

        fn addq(&mut self, data: u32) -> () {
                self.number += data;
        }

        fn get(&mut self) -> u32 {
                return self.number;
        }
}

pub fn scope_mutability()
{
        // y is valid here

        let mut y: Stack = Stack::default();
        y.setq(2);
        y.addq(2);
        y.addq(4);
        y.addq(4);
        
        println!("get: {}", y.get())

        
}