// Make it work
use std::mem::size_of_val;
fn main() {
    let c1: char = 'a'; // a = 4 bytes
    assert_eq!(size_of_val(&c1),4);

    let c2: char = '中'; 
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
} 

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// Make it work
fn main() {
    let c1: char = '中';
    print_char(c1);
} 

fn print_char(c: char) {
    println!("{}", c);
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

