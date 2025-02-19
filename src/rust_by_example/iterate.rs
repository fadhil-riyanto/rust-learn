use std::sync::Arc;
use std::thread;

pub fn runiterate()
{
        let data = Arc::new(vec![
                3, 4, 1, 9, 2
        ]);

        for _ in 0..10 {
                let y = Arc::clone(&data);

                // let handle = thread::spawn(f);
        }
}