// The type of string literal "hello, world" is &str, e.g let s: &str = "hello, world".

fn main() {
    let s: &str = "hello, world";

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let s: Box<&str> = "hello, world.".into();
    greetings(s);
}

fn greetings(s: &str) {
    println!("{}", s);
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
// String type is defined in std and stored as a vector of bytes (Vec), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.

fn main() {
    let mut s: String = String::from(""); // this is an empty string
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let s = String::from("hello");
    s.push(',');
    s.push_str(" Rust");
    s += "1";

    println!("{}", s);

}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let mut s: String = String::from("I like dogs"); // must be mutable to modify the string
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats"); // replace the word "dogs" with "cats" by using the replace method

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + s2.as_str(); // you can us as_str() to convert a string to a string slice or just an &
    assert_eq!(s3, "hello,world!");
    println!("{}", s3); // s3 owns the data
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let s = "hello, world";
    greetings(s)
}

fn greetings(s: String) {
    println!("{}", s)
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_



// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
