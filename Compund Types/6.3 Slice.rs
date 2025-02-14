//Slices are similar to arrays, but their length is not known at compile time, so you can't use slice directly.

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; // &[1, 2] held

    let s2: str = "hello, world";

    println!("Success!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// A slice reference is a two-word object, for simplicity reasons, from now on we will use slice instead of slice reference. 
// The first word is a pointer to the data, and the second word is the length of the slice. 
// The word size is the same as usize, determined by the processor architecture, e.g. 64 bits on an x86-64. 
// Slices can be used to borrow a section of an array, and have the type signature &[T].

fn main() {
    let arr: [char; 3] = ['中', '国', '人']; 

    let slice: &[char] = &arr[..2]; // ['中', '国'] held
    
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16); // slice holds 16 bytes because referencing using a slice is a two-word object (pointer and length) and each word is 8 bytes on a 64-bit system so it would be 4x4 = 16 bytes
    // 8 only passes when the slice is an array and not a reference to the array
    println!("Success!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    let slice: &[i32] = &arr[1..4]; // [2, 3, 4] held
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let s: String = String::from("hello");

    let slice1: &str = &s[0..2]; // &[h, e] held
    
    let slice2: &str = &s[..2]; // &[h, e] held
y
    assert_eq!(slice1, slice2); // both slices are equal

    println!("Success!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let s: &str = "你好，世界";

    let slice: &str = &s[0..3]; // [你] held - 3 bytes are needed because its an unicode character

    assert!(slice == "你");

    println!("Success!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// &String can be implicitly converted into &str.
// This is because &str is a slice of a string, and &String is a reference to a string.
// The same is true for &Vec<T> and &T.

fn main() {
    let mut s: String = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
    let letter: &str = first_letter(&s); // &String -> &str implicit conversion by compiler
    println!("the first letter is: {}", letter);

    // clear has to be called on a mutable reference, so we need to use `&mut String` type
    s.clear(); // removes all characters from the string

}
fn first_letter(s: &str) -> &str {
    &s[..1] 
}
