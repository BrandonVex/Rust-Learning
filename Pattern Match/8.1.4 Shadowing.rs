fn main() {
    let age: Option<i32> = Some(30);
    if let Some(age) = age { // Age is shadowed
       assert_eq!(age, 30));
    } 

    match age {
        // Match can also introduce a new shadowed variable :)
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}
    
 