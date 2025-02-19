
//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

enum Number { // enumerated
    Zero, // 0 implicitly
    One, // 1
    Two, // 2
}

enum Number1 {
    Zero = 0, // 0
    One, // 1
    Two, // 2
}

// C-like enum
enum Number2 {
    Zero = 5, // Can't use floating point values
    One, // implicit 6
    Two = 9, // 9
}


fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
    
    println!("{}", Number::One as u8); // this prints the value of One      

    println!("Success!");
} 

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

enum Message {
    Quit, // No additional data
    Move { x: i32, y: i32 }, // fields
    Write(String), // tuple like variant 
    ChangeColor(i32, i32, i32), // tuple like variant
}

fn main() {
    let msg1: Message = Message::Move{ x: 1, y: 2 }; // Instantiating with x = 1, y = 2 
    let msg2: Message = Message::Write(String::from("Hello World")); // Instantiating with "hello, world!"

    println!("Success!");
} 

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// We can get the data which an enum variant is holding by pattern match.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg: Message = Message::Move{x: 1, y: 1}; // instance of move variant

    if let Message::Move{ x: a, y: b} = msg { // pattern matching
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN!");
    }

    println!("Success!");
} 


//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

#[derive(Debug)] // must derive Debug to print the enum because its custom
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: [Message; 3] = [ // 3 message types are located in this array
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    println!("{:?}", msg); // :? allows us to print the enum because :? is a debug trait
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_

// Since there is no null in Rust, we have to use enum Option<T> to deal with the cases when the value is absent.
fn main() {
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five); /// some(6) return value
    let none: Option<i32> = plus_one(None);

    if let Some(n) = six { 
        println!("{}", n);

        println!("Success!");
    } else {
        panic!("NEVER LET THIS RUN！");
        
    }
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

