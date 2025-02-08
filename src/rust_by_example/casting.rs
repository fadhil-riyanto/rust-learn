pub fn casting() -> () {
        let decimal = 54.23838_f64;

        // nb: let conv2dec: u64 = decimal; //  this is implicit  
        let conv2dec: u64 = decimal as u64; // this is explicit

        // check lg than 256 bit

        // 1 0 0 1  0 0 0 1   1 0 1 0  0 0 1 1
        let ufor16: u16 = 37283;

        // 0 0 0 0  0 0 0 0   1 0 1 0  0 0 1 1
        let ufor8: u8 = ufor16 as u8;

        // this is char cast
        let mynum = 61.9;
        let mynum_dec = mynum as u8;
        let mynum_chr = mynum_dec as char;

        println!("{conv2dec}");
        println!("{ufor8}");
        println!("{mynum_dec}");
        println!("{mynum_chr}");
        
}