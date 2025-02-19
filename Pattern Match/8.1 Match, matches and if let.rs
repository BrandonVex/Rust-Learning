enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire: Direction = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("West"),
    };
}

//------------------------------------------------


fn main() {
    let boolean: bool = true;

    let binary: u8 = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}

//------------------------------------------------

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
} 

fn show_message(msg: Message) {
    match msg {
        Message::Move {x: a, y: b} => { // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(r, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants")
    }
}

//------------------------------------------------

