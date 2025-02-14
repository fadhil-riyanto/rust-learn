

macro_rules! eight {
        () => {
                4 + 4
        }
}

macro_rules! myrule34 {
    ($a:expr) => {
                1 + $a
    };
}

macro_rules! myblock_tojson {
    ($body:block) => {
        $body
    };
}

pub fn testmacro()
{
        println!("{}", eight!() + myrule34!(9 + 9));

        let y = vec![3, 4, 2, 0, 1];
        let data = myblock_tojson! {
                {
                        let a: u32 = 3 + 3;
                        println!("{}", a);
                }
        };
}