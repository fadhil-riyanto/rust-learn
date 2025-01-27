struct Counter {
        count: usize,
}

impl Counter {
        fn new() -> Counter {
                let mut a: Counter = Counter{
                        count: 1
                };

                return a; 
        }
}

// impl Iterator for Counter {
//         type Item = usize;
// }

pub fn do_iter2() -> () {
        let a: Counter = Counter::new();


        println!("{}", a.count);
}