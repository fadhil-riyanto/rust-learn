pub fn freezing() -> () {
        let mut y = 10;

        // {
        //         // this is shadowing

        //         let y = y;

        //         y = 192;
        // }
        y = 123;

        println!("{y}");
}