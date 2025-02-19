fn main() {
    for n in 1..100 { // 1 to 99

        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("{}", n);

    println!("Success!");
} 

// -------------------------------------------------------------

fn main() {
    let names [String; 2] = [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        println!("{}", name); // auto dereferences
    }

    println!("{:?}", names);

    let numbers: [i32; 3] = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        println!("{}", n);
    }
    
    println!("{:?}", numbers);
} 


// -------------------------------------------------------------

fn main() {
    let a: [i32; 4] = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() { // enumerate() returns a tuple and iter() give the ability to iterate
        println!("The {}th element is {}",i+1,v);
    }
}