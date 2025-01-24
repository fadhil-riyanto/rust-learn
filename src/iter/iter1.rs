pub fn iter_enumerate_test()
{
        iter_one();
        iter_bytes_oldver();
        iter_new();
}

fn iter_one()
{
        let a = [3, 4, 2, 1, 9];
        let mut iter = a.iter();

        // println!("{:?}", a);
}

fn iter_bytes_oldver()
{
        let str: String = String::from("random old");

        for i in 0..str.len() {
                println!("--> {}", str.as_bytes()[i] as char);
        }
        
}

fn iter_new()
{
        let str: String = String::from("random old");

        let strb: &[u8] = str.as_bytes();

        println!("{:?}", strb[0] as char);
}