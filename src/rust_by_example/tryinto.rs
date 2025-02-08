
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                        return Result::Ok(EvenNumber(value));
                } else {
                        return Result::Err(());
                }
        }
}

pub fn tryinto_main() 
{
        assert_eq!(EvenNumber::try_from(4), Ok(EvenNumber(4)));
        assert_eq!(EvenNumber::try_from(3), Result::Err(()));
        
        // tryinto
        let res: Result<EvenNumber, ()> = 8_i32.try_into();
        assert_eq!(EvenNumber(8), Ok(res))
}