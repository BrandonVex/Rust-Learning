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
    let s = "hello, world"; // &str to String
    greetings(s) // .to_string() or .to_owned() can be used to convert a string slice to a String
}

fn greetings(s: String) {
    println!("{}", s)
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

//🌟🌟 We can use String::from or to_string to convert a &str to String
// Use two approaches to fix the error and without adding a new line
fn main() {
    let s: String = "hello, world".to_string(); 
    let s1: &str = &s; //&String -> &str - can use .as_str() or &

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape); // double backslash is used to escape the backslash 

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

//🌟🌟🌟 Sometimes there are just too many characters that need to be escaped or it's just much more convenient to write a string out as-is. This is where raw string literals come into play.
/* Fill in the blank and fix the errors */
fn main() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";  // raw string is where no escapes are allowed
    // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = __;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// Want a string that's not UTF-8? (Remember, str and String must be valid UTF-8). Or maybe you want an array of bytes that's mostly text? Byte strings to the rescue!

use std::str;

fn main() {
    // Note that this is not actually a `&str`
    let bytestring: &[u8; 21] = b"this is a byte string";

    // Byte arrays don't have the `Display` trait, so printing them is a bit limited
    println!("A byte string: {:?}", bytestring);

    // Byte strings can have byte escapes...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...But no unicode escapes
    // let escaped = b"\u{211D} Is not allowed";
    println!("Some escaped bytes: {:?}", escaped);


    // Raw byte strings work just like raw strings
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // Converting a byte array to `str` can fail
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // Byte strings don't have to be UTF-8
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" In SHIFT-JIS

    // But then they can't always be converted to `str`
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}

// byte strings are good for debugging and for storing binary data

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// 🌟🌟🌟 You can't use index to access a char in a string, but you can use slice &s1[start..end].
// you cant use indexing to access a char in a string because it is a vector of bytes and not a vector of chars
// you have to use string slices to access a char in a string

fn main() {
    let s1: String = String::from("hi,中国");
    let h: &str = &s1[0..1]; // 0 .. 1 is a range that includes the first element but excludes the second element 
    assert_eq!(h, "h");

    // char is 4 byes in UTF-8 format 
    let h1 = &s1[3..6];  // because the character is 3 bytes long, you have to include the 3rd byte to get the character so 3..6 is the correct instead of 3..5
    assert_eq!(h1, "中");

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    // Fill the blank to print each char in "你好，世界"
    // chars will put each character into an iterator
    // use for loop to iterate over the iterator
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

//You can use utf8_slice to slice UTF8 string, it can index chars instead of bytes.
use utf8_slice;
fn main() {
    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 5);
    // Will equal "🚀"
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_