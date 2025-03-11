// limit of lifetimes

#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }

    fn share(&self) {}
}

fn limit_of_lifetimes() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share(); // this basically return itself
    // foo.share();
    println!("{:?}", loan);
}

// immutable ctx 
#[derive(Debug)]
struct a {
    b: u32,
}

impl a {
    fn getme(&self) -> &Self  {
        &*self
    }
    fn caller(&self) {
        
    }

}
fn immutable_ctx() {
    let z = a{
        b: 2,
    };


    let y = z.getme();
    y.caller();
}

// Higher-Rank Trait Bounds (HRTBs)

struct Clousure {
    data: u32
}

fn HRTBs() {

}

pub fn run() {
    limit_of_lifetimes();
    immutable_ctx();
    HRTBs();
}