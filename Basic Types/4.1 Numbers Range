fn main() {
    let mut sum: i32 = 0; // -3, -5, -6, -5
    
    for i in -3..2 { //2 is excluded so if you need 2 use "3" pr "=2"
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' { // = side will mean "z" is included
        println!("{}",c as u8);
    }
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_ 

use std::ops::{Range, RangeInclusive}; //importing these modules
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 }); // 5 is excluded
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}