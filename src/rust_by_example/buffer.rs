
/// todo
/// one trait, for many struct
/// 
/// one trait, 3 implementation 
/// 
trait My4StructTypes {
        fn getMsg(&self) -> ();
}

struct A {
        data: String,
}
struct B {
        data: String,
}
struct C {
        data: String,
}

// IMPL 
impl My4StructTypes for A {
        fn getMsg(&self) -> () {
            println!("A says{}", self.data);
        }
}

impl My4StructTypes for B {
        fn getMsg(&self) -> () {
            println!("B says{}", self.data);
        }
}

impl My4StructTypes for C {
        fn getMsg(&self) -> () {
            println!("c says{}", self.data);
        }
}

fn doing_something(xcc: &impl My4StructTypes) {
        xcc.getMsg();
}

pub fn do_reserve()
{
        let xa = A {
                data: "thisisa".to_string()
        };

        let xb = B {
                data: "thisisB".to_string()
        };

        let xC = C {
                data: "thisisC".to_string()
        };

        doing_something(&xb);

}