use std::fmt;

#[derive(Debug)]
struct x {
    message: String,
}


#[derive(Debug)]
struct xaps {
    message: String,
}

/// ni std::error::Error implement display + debug (intina libfmt + display)
/// disini implement pertama, struct x kpd error
impl std::error::Error for x {}

// next implement display, agar errornya bisa di print
impl fmt::Display for x {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// second impl
impl std::error::Error for xaps {}

impl fmt::Display for xaps {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "xaps {}", self.message)
        }
}

/// create box (aslinya cuma alloc di heap)
/// lalu karna std error trait diimplement banyak  struct, jadi kita pake dyn (dynamic)
fn test1() -> Result<(), Box<dyn std::error::Error>> {
    let y: x = x {
        message: "error lol".to_string(),
    };

    let z: xaps = xaps {
        message: "error zaps".to_string(),
    };
    return Err(Box::new(z));
}

pub fn test_box() {
    match self::test1() {
        Ok(x) => println!("okay"),
        /// proses unpack boxnya, dari yg awalnya di heap, karna kitamasukkin ke Box::new tuh
        /// maka kita return & cast pakai downcast ref
        Err(x) => println!("data {}", x.downcast_ref::<xaps>().unwrap().message),
    }
}
