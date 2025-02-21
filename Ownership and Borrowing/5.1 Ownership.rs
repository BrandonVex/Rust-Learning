fn main() {
    let x: String = String::from("Hello world");
    let y: String = x.clone();
    println!("{}, {}",x, y);
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-__

// Don't modify code in main!
fn main() {
    let s1: String = String::from("Hello world");
    let s2: String = take_ownership(s1); //s2 is owned of data because s1 is moved to take_ownership

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> () { //anotate the return type
    println!("{}", s); // print s
    s // return s
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let s: String = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes(); //as_bytes() returns a Vec<u8> because it is a vector of bytes which stores the ASCII values of the characters in the string
    s
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// Fix the error without removing any code
fn main() {
    let s: String = String::from("Hello World");

    print_str(s.clone()); //argument of S

    println!("{}", s); // can't use s here because it was moved to print_str
}                      // can be used if we clone it

fn print_str(s: String)  { //print out the string
    println!("{}",s)
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let x: (i32, i32, (), &str) = (1, 2, (), "hello"); //tuple with 4 elements & assign the elements in the tuple 
    // the &str is hard coded into the binary itself - immutable reference to a string literal - known at compile time
    let y: (i32, i32, (), &str) = x.;
    println!("{:?}, {:?}", x, y);
}
// fixed size is copied implicitly

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let s: String = String::from("Hello, ");
    
    let mut s1 = s; //mut makes s1 mutable
    // mutable means that the value can be changed after it is bound to a name
    s1.push_str("world!");

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let x: Box<i32> = Box::new(5); //box allows you to store data on the heap rather than the stack
    
    let mut y: Box<i32> = Box::new(1); // Box is a smart pointer that points to data on the heap
   
    *y = 4; //we use dereference operator to change the value of y to 4

    // you have to use the dereference operator to access the value of a Box
    
    assert_eq!(*x, 5);

    println!("Success!");
}