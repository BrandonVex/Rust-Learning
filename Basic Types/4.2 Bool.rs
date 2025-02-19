
// Make println! work
fn main() {
    let _f: bool = false;

    let t: bool = true;
    if t {
        println!("Success!");
    }
} 

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// Make it work
fn main() {
    let f: bool = false;
    let t: bool = true && false; // false
    assert_eq!(t, f);

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

