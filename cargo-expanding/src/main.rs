use std::collections::HashMap;

macro_rules! my_block_macro {
    ($body:block) => {{
        println!("Before the block");
        let result = $body;
        println!("After the block");
        result
    }};
}

fn test1() {
    let value = my_block_macro! {
        {
            println!("Inside the macro block");
            42
        }
    };

    println!("Macro returned: {}", value);

    // test ownership

    let b = 9;

    {}
}

#[warn(unused_macros)]
macro_rules! creat_var {
    ($name:ident, $value:expr) => {
        let $name = $value;
        println!("created var with name {}", stringify!($name));
    };
}

macro_rules! calc {
    ($e:expr) => {
        let res = $e;
        println!("{}", res);
    };
}

macro_rules! exec {
    ($e:stmt) => {
        println!("i want to execute this {}", stringify!($e));
        $e
    };
}

macro_rules! mixed_var {
    ($name:ident, $type:ty, $val:expr) => {
        let $name: $type = $val;
    };
}

// some rule, try initialize a macro which a non dynamic
macro_rules! path_hashmap {
    ($mypath:path, $tipe:ty, $initialkey:expr, $initial_val:expr) => {{
        // Enclose in a block to return the HashMap

        let mut book_reviews: HashMap<&str, &str> = HashMap::new();
        book_reviews.insert($initialkey, $initial_val);

        println!("{:?}", book_reviews);
        book_reviews // Return the HashMap
    }};
}

macro_rules! testpat {
    ($e:expr, $matchme:pat) => {
        match $e {
            $matchme => println!("im match"),
            _ => println!("im default"),
        }
    };
}

fn test2() {
    let reviews: HashMap<&str, &str> =
        path_hashmap!(std::collections, HashMap, "The Great Gatsby", "Excellent");

    // Now you can use the 'reviews' HashMap
    println!("The returned map has {} elements", reviews.len());
    println!(
        "Review for 'The Great Gatsby': {:?}",
        reviews.get("The Great Gatsby")
    );
}

fn main() {
    // self::test1();
    self::test2();

    creat_var!(aaaa, 2);

    println!("bbb {}", aaaa);

    calc!(3 + 3 + (1 + 1));
    exec!(let a = 9);
    mixed_var!(hhhh, u32, 981);
    println!("hhh value is {}", hhhh);

    path_hashmap!(std::collections, HashMap, "a", "b");

    testpat!(4 + 4, 8);
}