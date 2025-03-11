use std::cmp::{Ord, PartialOrd, Eq, PartialEq};


struct Point {
    x: i32,
    y: i32,
}

// this section is eq
impl Eq for Point {}

fn eqme() {
    let apoint = Point {
        x: 2,
        y: 3,
    };

    let bpoint = Point {
        x: 2,
        y: 4,
    };

    // assert_eq!(apoint == bpoint, true);
    println!("eq: {:?}", apoint == bpoint);
}


// this is partialeq
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

fn partialeq() {
    let apoint = Point {
        x: 2,
        y: 3,
    };

    let bpoint = Point {
        x: 22,
        y: 4,
    };

    // assert_eq!(apoint == bpoint, true);
    println!("partialeq: {:?}", apoint == bpoint);
}

pub fn run() {
    self::partialeq();
    self::eqme();
}