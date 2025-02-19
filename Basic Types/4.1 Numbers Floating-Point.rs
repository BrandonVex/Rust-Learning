fn main() {
    let x: f64 = 1_000.000_1; // f64 is default floating type
    let y: f32 = 0.12; // f32
    let z: f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_ 

fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32); // floating point precision is needed

    println!("Success!");
}

// f32 is less precises (can also use  "as f32")

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_ 




