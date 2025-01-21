use rand::Rng;
use rand::thread_rng;

struct Randomdata {
        seed: u32,
        seed2: u32,
        seed3: u32,
        pad: u32
}

pub fn do_full_fmt_stuff()
{
        do_intfmt();
        do_readstruct();
}

fn do_intfmt()
{
        let r: u64 = 18446744073709551615;
        println!("this is linus {}, started from ", r)
}

fn do_readstruct()
{
        let mut rng = thread_rng();
        let x = Randomdata{
                seed: rng.gen_range(9..1129),
                seed2: 9,
                seed3: 58,
                pad: 3
        };

        println!("random: {}", x.seed * x.seed2 + x.seed3 ^ x.pad)
}