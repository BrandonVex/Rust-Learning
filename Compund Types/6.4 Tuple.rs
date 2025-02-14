//  Elements in a tuple can have different types. Tuple's type signature is (T1, T2, ...), where T1, T2 are the types of tuple's members.

fn main() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// Members can be extracted from the tuple using indexing.
fn main() {
    let t: (&str, &str, &str) = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface"); //t.1 is the second element of the tuple - t.2 is the third element of the tuple

    println!("Success!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// long tuples cant be printed
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13); // 12 is the maximum for printable tuples
    // can be used, just not printed over 13 elements
    println!("too long tuple: {:?}", too_long_tuple);
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let tup: (i32, f64, &str) = (1, 6.4, "hello");

    // Destructuring the tuple using pattern matching
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let (x, y, z);

    // Destructuring the tuple using pattern matching
    // we dont need to use let keyword to declare the variables because its already declared in line 1
    (y, z, x) = (1, 2, 3);
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// Tuples can be used as function arguments and return values
fn main() {
    // sum_multiply will be 2,3 because 2+3 = 5 and 2*3 = 6 and x = 5 when y = 6
    let (x, y) = sum_multiply((2,3));  // you need parantheses around the tuple because you are passing a tuple as an argument

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1) // return a tuple
}

