use crate::dd;


struct MyChainer {
        cur_int: Vec<u16>,
        len:    u16
}

impl MyChainer {
        fn get_idx(&self, i: usize) -> u16 {
                self.cur_int[i]
        }

        fn do_push(&mut self, val: u16) -> &mut Self {
                self.cur_int.push(val);
                self.len = self.len + 1;

                self
        }
}

fn do_own_ret(len: u16) -> MyChainer {
        MyChainer {
                cur_int: Vec::new(),
                len
        }
}

pub fn do_vec_chr() -> () {
        let mut MyChainer = MyChainer {
                cur_int: Vec::new(),
                len: 0,
        };

        let mut otherChainer = self::do_own_ret(0);

        &otherChainer.do_push(3)
                .do_push(5)
                .do_push(8);

        dd!(otherChainer.cur_int);
        



}