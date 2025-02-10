use std::convert::AsRef;
use std::fs;
use std::path::Path;

fn doprint(data: &String) {
    print!("{}", *data);
}

fn simple_borrow() {
    let x: String = "aaaaddaa".to_string();
    doprint(&x);

    println!("{x}")
}

fn actually()  {
    let path = Path::new("dwedwewed.txt");
    let newpath  = Path::new("dwedwewed.txt");

    fs::rename(path, "jjjjj.txt");

    println!("done");

}

struct pair<T> {
        amount: T,
        usdt: u32,
}

trait calc_balance {
        fn len(&self) -> u32;
}

enum currency {
        ETH(u32),
        BTC(u32),
        SOL(u32),
}




pub fn do_asref() {
    simple_borrow();

}
