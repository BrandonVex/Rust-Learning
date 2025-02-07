//Partial move
// Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be used at the same time. Doing this will result in a partial move of the variable, which means that parts of the variable will be moved while other parts stay. In such a case, the parent variable cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.


fn main() {
    #[derive(Debug)]
    struct Person { //custom data type
        // template for the struct
        name: String, //name of the person || member 1
        age: Box<u8>, //age of the person || member 2
    }

    let person: Person = Person { //using the struct template to give it a value
        name: String::from("Alice"), // giving the name a value
        age: Box::new(20), // giving the age a value
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person; //destructuring the struct

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    printl

// -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

fn main() {
    let t: (String, Stirng) = (String::from("hello"), String::from("world"));
    // Tuples has 0 based indexing
    let _s: String = t.0; // _s holds the first element of the tuple which is "hello"
 
    // Modify this line only, don't use `_s`
    println!("{:?}", t.1); // print out the second element of the tuple because T still owns the data (index 1 data)
 }

 // -_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

 fn main() {
    let t: (String, String)= (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     let (s1, s2) = t.clone();
 
     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }