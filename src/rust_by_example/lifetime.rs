// lifetime reproduce

use rand::Rng;

// fn lifelong_ref(x: &str, y: &str) -> &str {
fn lifelong_ref<'a, 'b>(x: &'a str, y: &'a str, z: &'b u32) -> &'b str {
    let mut rng = rand::thread_rng();
    let yno = rng.gen_range(0..2);

    println!("{}", yno);
    if yno == 1 {
        x
    } else {
        y
    }
}

pub fn run() -> () {
    let a = "Aaaaa".to_string();
    let a2 = "BBBBBBB".to_string();
    let b = lifelong_ref(&a, &a2);
    println!("{} ||| {}", a, b);
}
