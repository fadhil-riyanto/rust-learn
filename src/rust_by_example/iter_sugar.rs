fn stenovia() {
    let values = vec![1, 2, 3, 4, 5];

    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                let next;
                match iter.next() {
                    Some(val) => next = val,
                    None => break,
                };
                let x = next;
                let () = {
                    println!("{x}");
                };
            },
        };
        result
    }

    let v = [1, 2, 3];
    let mut iter = v.into_iter();
}

// fn iter_old()
// {
//     let myvec = vec![1, 2, 3, 4, 5];
//     // let vector = 
// }

fn closure()
{
        let mut a = 0;
        let b = |x: &u32| *x + 1;

        println!("{}", b(&a));
        println!("{}", b(&a));
        println!("{}", b(&a));
}

pub fn run_sugar() {
        stenovia();
        closure();
        // iter_old();
}
