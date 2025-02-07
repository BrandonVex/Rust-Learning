fn main() {
   let x: i32  = 5;
   // Fill the blank
   let p: &i32 = &x; // p is a reference to x

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

// ---------------------------------------------

fn main() {
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, *y); // dereference y because y is a reference to x and we want to compare the value of x to 5

    println!("Success!");
}

// ---------------------------------------------

fn main() {
    let mut s: String = String::from("hello, ");

    borrow_object(&s); // reference to the string

    println!("Success!");
}

fn borrow_object(s: &String) {}

// ---------------------------------------------

fn main() {
    let mut s: String = String::from("hello, ");

    push_str(&mut s); // pass a mutable reference to the string

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

// ---------------------------------------------

fn main() {
    let mut s: String = String::from("hello, ");

    let p: &mut String = &mut s; // p is a mutable reference to s
    
    p.push_str("world");

    println!("Success!");
}

// --------------------------------------------- 
// ref can be used to take references to a value, similar to &.

fn main() {
    let c = '中'; // char type

    let r1: &char = &c; // r1 is a reference to char c 
    // Fill the blank，dont change other code
    let ref r2 = c; // r2 is a reference to char c but using ref keyword

    assert_eq!(*r1, *r2); // dereference r1 and r2 to compare the values
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2)); 

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String { // r is a reference to a char 
    format!("{:p}", r) // return the memory address of r
    // format! is a macro that returns a String
}

// ---------------------------------------------

fn main() {
    let mut s: String = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("s: {}", s); // s is borrowed by r1 and r2

    println!("Success!");
}

// ---------------------------------------------

fn main() {
    // Fix error by modifying this line
    let mut s: String = String::from("hello, ");

    borrow_object(&mut s); // string must be mutable to pass a mutable reference

    println!("Success!");
}

fn borrow_object(s: &mut String) {}

// ---------------------------------------------

fn main() {
    let mut s: String = String::from("hello, ");

    borrow_object(&s); // immutable references to care if the value is mutable or not
    
    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}

// ---------------------------------------------

fn main() {
    let mut s: String = String::from("hello, ");

    let r1: &mut String = &mut s;
    r1.push_str("world");

    let r2: &mut String = &mut s;
    r2.push_str("!");
    
    //println!("{}",r1);
}

// ---------------------------------------------

fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time

    println!("{}, {}", r1, r2);

}