// We must specify concrete values for each of the fields in struct.

struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age: u8 = 30; // u8 needs to be used because the type in the struct is u8
    let p: Person = Person { // this is person type because the struct is person
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!("Success!");
} 


//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// Tuple struct looks similar to tuples, it has added meaning the struct name provides but has no named fields. It's useful when you want to give the whole tuple a name, but don't care about the fields's names
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let v: Point = Point (0, 127, 255);
    check_color(v);

    println!("Success!");
}   

fn check_color(p: Point) { //argument is Point type
    let Point(x, _, z) = p; //underscore is used when you dont need to use the value
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
 }

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// You can make a whole struct mutable when instantiating it, but Rust doesn't allow us to mark only certain fields as mutable.
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age: u8 = 18;
    let mut p: Person = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18? 
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// Using field init shorthand syntax to reduce repetitions.
struct Person {
    name: String,
    age: u8,
}
fn main() {
    println!("Success!");
} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name,
    }
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

//You can create instance from other instance with struct update syntax
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let u1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2: User = set_email(u1);

    println!("Success!");
} 

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// We can use #[derive(Debug)] to make a struct printable.
#[derive(Debug)] // Debug is a trait that enables the use of the {:?} formatter

// struct rectanble implements the Debug trait
// now you can use debug print to print the struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale: u32 = 2;
    let rect1: Rectangle = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be used at the same time. 
// Doing this will result in a partial move of the variable, which means that parts of the variable will be moved while other parts stay.
// In such a case, the parent variable cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn main() {
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name: String = f.name; // we cant use f.name or f after this line because this was a partial move

    println!("{}, {},",_name, f.data);
} 


//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_



//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_