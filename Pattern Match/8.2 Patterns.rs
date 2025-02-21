// Use | to match several values, use ..= to match an inclusive range.

fn main() {}
fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        2 | 3 | 4 | 5 => println!("match 2 -> 5"), // this is a pattern
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => { //any other value we dont care about
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

// The match statement is used to match the value of a variable against a pattern.
// In this case, the pattern is a series of values separated by the | operator.
// If the value of n is 2, 3, 4, or 5, then the code block will execute.

// -------------------------------------------------------------------------------------------------

//The @ operator lets us create a variable that holds a value, at the same time we are testing that value to see whether it matches a pattern.
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Fill in the blank to let p match the second arm
    let p: Point = Point { x: 3, y: 20 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// The @ operator is used to create a variable that holds a value, at the same time we are testing that value to see whether it matches a pattern.
// In this case, the pattern is Point { x: 0..=5, y: y@ (10 | 20 | 30) }, which matches the value of p if x is between 0 and 5 and y is 10, 20, or 30.
// The variable y holds the value of y if the pattern matches.

// -------------------------------------------------------------------------------------------------

enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg: Message = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id:  id @ 3..=7, // the @ symbol is used to create a variable that holds a value, at the same time we are testing that value to see whether it matches a pattern. - it also destructs the value
        } => println!("Found an id in range [3, 7]: {}", id),
        Message::Hello { id: newid @ (10 | 11 | 12) } => { // pattern matching with the @ operator
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

// The @ operator is used to create a variable that holds a value, at the same time we are testing that value to see whether it matches a pattern.
// In this case, the pattern is Message::Hello { id:  id @ 3..=7 }, which matches the value of msg if id is between 3 and 7.
// The variable id holds the value of id if the pattern matches.
// the pattern also must be withing paranthesis when using the @ operator

// -------------------------------------------------------------------------------------------------

// A match guard is an additional if condition specified after the pattern in a match arm that must also match, along with the pattern matching, for that arm to be chosen.
fn main() {
    let num: Option<i32> = Some(4);
    let split: i32 = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

// The match guard is an additional if condition specified after the pattern in a match arm that must also match, along with the pattern matching, for that arm to be chosen.
// In this case, the pattern is Some(x) if x < split, which matches the value of num if it is Some(x) and x is less than split.
// If the pattern matches, the code block will execute.

// -------------------------------------------------------------------------------------------------

// Ignoring remaining parts of the value with ..

fn main() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}

// The .. operator is used to ignore the remaining parts of the value.
// In this case, the pattern is (first, .., last), which matches the value of numbers if it is a tuple with at least two elements.
// The first element is assigned to first, and the last element is assigned to last.

// -------------------------------------------------------------------------------------------------

// Using pattern &mut V to match a mutable reference requires you to be very careful, due to V being a value after matching.

fn main() {
    let mut v: String = String::from("hello,");
    let r: &mut String = &mut v;

    match r {
        value => value.push_str(" world!") 
    }
}

// The pattern &mut V is used to match a mutable reference.
// In this case, the pattern is value, which matches the value of r.
// If the pattern matches, the code block will execute.

