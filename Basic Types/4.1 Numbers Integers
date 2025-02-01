fn main() {
    let x: i32 = 5;
    let mut y = 5; // i32 is base

    y = x;
    
    let z: i32 = 10; 

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_ 

fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_ 

fn main() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>()) // "i32"
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_ 

fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_ 

fn main() {
   let v1 = 251_u16 + 8;
   let v2 = i16::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_ 

fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert!(v == 1597);

    println!("Success!");
}
