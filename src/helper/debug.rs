#[macro_export]
macro_rules! dd {
        ($a:expr) => {
                {
                        println!("exec: {:?}", $a);
                }
        }
}