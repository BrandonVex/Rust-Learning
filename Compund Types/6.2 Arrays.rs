fn main() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // you need to specify the type of the array and the length of the array

    // Modify the code below to make it work
    assert!(arr.len() == 5); // 5 is the length of the array

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12); // 3 * 4 = 12 bytes

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
//🌟 All elements in an array can be initialized to the same value at once.
fn main() {
    // Fill the blank
    let list: [i32; 100] = [1;100] ; // [1;100] means 100 elements of 1

    assert!(list[0] == 1); // access the first element of the array
    assert!(list.len() == 100);

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

//🌟 All elements in an array must be of the same type
fn main() {
    let _arr: [i32; 3] = [1, 2, 3];

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let arr: [char; 3] = ['a', 'b', 'c'];
    
    let ele = arr[0]; // access the first element of the array

    assert!(ele == 'a');

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

//🌟 Out of bounds indexing causes panic
fn main() {
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap(); // get will return an option, so you have to unwrap it to get the value
 
    // But indexing is not safe
    let _name1 = &names[1];

    println!("Success!");
}


// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_
