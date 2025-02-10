# Personal note about rust
This is big fat notebot, absolutely... 

### Rule 1: 
```i32::from_str_radix("this is string")```

throwing `Result<i32, ParseIntError>` enum, use
Result::ok in case value is true, or Result::Err in case the value is wrong.

example 1: 
```rust
let b: Result<i32, ParseIntError> = i32::from_str_radix("12a", 10);
if let Err(e) = &b {
        println!("Failed conversion to i32: {e}");
}

if let Ok(e) = &b {
        println!("result: {e}");
}
```

nb: deference b because we dont want to borrow it, useful for futures operation. 

footnotes:
- https://doc.rust-lang.org/nightly/std/num/struct.ParseIntError.html
- reference/expressions/operator-expr.html#the-question-mark-operator
- https://doc.rust-lang.org/reference/items/enumerations.html
- https://stackoverflow.com/questions/77166746/how-to-print-error-from-a-result-the-right-way
- https://doc.rust-lang.org/beta/rust-by-example/flow_control/let_else.html


### Rule 2: 
unwrap() return panic **if** none

- https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html