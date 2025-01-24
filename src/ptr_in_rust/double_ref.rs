pub fn double_ref()
{
        let mut x: u8 = 4;
        
        // let r3: &mut u8 = &mut x;

        // in this case, x is deferenced by two variable
        // gives a chance to conflick when do multithread tasks
        // solution
        {
                let r3: &mut u8 = &mut x;
                // we call drop(r3)
        }

        let r1: &mut u8 = &mut x;

        println!("{r1}");
}

pub fn double_ref_immutability()
{
        let mut x: u8 = 4;
        let mut y = &x;

        y = &9;

        // println!("{:p} {:p}", &x, y);
        println!("{} {}", x, y);
}