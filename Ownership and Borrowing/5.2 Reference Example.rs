fn main() {
    let s1 = String::from("Hello world");

    let len = calculate_length(&s1); //passing a reference to the string

    println!("The length of '{}' is {}.", s1, len); // s1 is still valid here
}

fn calculate_length(s: &String) -> usize { //s is a reference to a String
    s.len() //return the length of the string
}

// mutable example 

fn main() {
    let mut s = String::from("hello");

    change(&mut s); //passing a mutable reference to the string

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); //pushing a string to the string
}

// mutable reference can only have one at a time in a scope